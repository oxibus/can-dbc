use can_dbc_pest::{Pair, Rule};

use crate::ast::AttributeValueForObjectType;
use crate::parser::{parse_next_inner_str, validated_inner, DbcError};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct AttributeValueForObject {
    pub name: String,
    pub value: AttributeValueForObjectType,
}

impl TryFrom<Pair<'_, Rule>> for AttributeValueForObject {
    type Error = DbcError;

    /// Parse attribute value: `BA_ attribute_name [object_type] object_name value;`
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut pairs = validated_inner(value, Rule::attr_value)?;

        Ok(Self {
            name: parse_next_inner_str(&mut pairs, Rule::attribute_name)?,
            value: pairs.try_into()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{AttributeValue, MessageId};
    use crate::test_helpers::*;

    #[test]
    fn network_node_attribute_value_test() {
        let def = r#"
BA_ "AttrName" BU_ NodeName 12;
"#;
        let exp = AttributeValueForObject {
            name: "AttrName".to_string(),
            value: AttributeValueForObjectType::NetworkNode(
                "NodeName".to_string(),
                AttributeValue::Uint(12),
            ),
        };
        let val = test_into::<AttributeValueForObject>(def.trim_start(), Rule::attr_value);
        assert_eq!(val, exp);
    }

    #[test]
    fn integer_and_float_attributes() {
        // attribute with a fractional part ".0" is parsed as Double
        let def = r#"
BA_ "Attribute" BU_ NodeName 12;
"#;
        let exp = AttributeValueForObject {
            name: "Attribute".to_string(),
            value: AttributeValueForObjectType::NetworkNode(
                "NodeName".to_string(),
                AttributeValue::Uint(12),
            ),
        };
        let val = test_into::<AttributeValueForObject>(def.trim_start(), Rule::attr_value);
        assert_eq!(val, exp);

        // negative attribute value without a fractional part is parsed as I64
        let def = r#"
BA_ "Attribute" BU_ NodeName -12;
"#;
        let exp = AttributeValueForObject {
            name: "Attribute".to_string(),
            value: AttributeValueForObjectType::NetworkNode(
                "NodeName".to_string(),
                AttributeValue::Int(-12),
            ),
        };
        let val = test_into::<AttributeValueForObject>(def.trim_start(), Rule::attr_value);
        assert_eq!(val, exp);

        // positive attribute value without a fractional part is parsed as I64
        let def = r#"
BA_ "Attribute" BU_ NodeName 12;
"#;
        let exp = AttributeValueForObject {
            name: "Attribute".to_string(),
            value: AttributeValueForObjectType::NetworkNode(
                "NodeName".to_string(),
                AttributeValue::Uint(12),
            ),
        };
        let val = test_into::<AttributeValueForObject>(def.trim_start(), Rule::attr_value);
        assert_eq!(val, exp);

        // positive attribute value without a fractional part is parsed as I64
        let def = r#"
BA_ "Attribute" BU_ NodeName 12.1;
"#;
        let exp = AttributeValueForObject {
            name: "Attribute".to_string(),
            value: AttributeValueForObjectType::NetworkNode(
                "NodeName".to_string(),
                AttributeValue::Double(12.1),
            ),
        };
        let val = test_into::<AttributeValueForObject>(def.trim_start(), Rule::attr_value);
        assert_eq!(val, exp);
    }

    #[test]
    fn message_definition_attribute_value_test() {
        let def = r#"
BA_ "AttrName" BO_ 298 13;
"#;
        let exp = AttributeValueForObject {
            name: "AttrName".to_string(),
            value: AttributeValueForObjectType::MessageDefinition(
                MessageId::Standard(298),
                Some(AttributeValue::Uint(13)),
            ),
        };
        let val = test_into::<AttributeValueForObject>(def.trim_start(), Rule::attr_value);
        assert_eq!(val, exp);
    }

    #[test]
    fn signal_attribute_value_test() {
        let def = r#"
BA_ "AttrName" SG_ 198 SGName 13;
"#;
        let exp = AttributeValueForObject {
            name: "AttrName".to_string(),
            value: AttributeValueForObjectType::Signal(
                MessageId::Standard(198),
                "SGName".to_string(),
                AttributeValue::Uint(13),
            ),
        };
        let val = test_into::<AttributeValueForObject>(def.trim_start(), Rule::attr_value);
        assert_eq!(val, exp);
    }

    #[test]
    fn env_var_attribute_value_test() {
        let def = r#"
BA_ "AttrName" EV_ EvName "CharStr";
"#;
        let exp = AttributeValueForObject {
            name: "AttrName".to_string(),
            value: AttributeValueForObjectType::EnvVariable(
                "EvName".to_string(),
                AttributeValue::String("CharStr".to_string()),
            ),
        };
        let val = test_into::<AttributeValueForObject>(def.trim_start(), Rule::attr_value);
        assert_eq!(val, exp);
    }

    #[test]
    fn raw_attribute_value_test() {
        let def = r#"
BA_ "AttrName" "RAW";
"#;
        let exp = AttributeValueForObject {
            name: "AttrName".to_string(),
            value: AttributeValueForObjectType::Raw(AttributeValue::String("RAW".to_string())),
        };
        let val = test_into::<AttributeValueForObject>(def.trim_start(), Rule::attr_value);
        assert_eq!(val, exp);
    }
}
