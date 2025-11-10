use can_dbc_pest::{Pair, Rule};

use crate::parser::{expect_empty, inner_str, next, next_optional_rule, next_rule, DbcError};
use crate::AttributeValueType;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AttributeDefinition {
    Message(String, AttributeValueType),
    Node(String, AttributeValueType),
    Signal(String, AttributeValueType),
    EnvironmentVariable(String, AttributeValueType),
    Plain(String, AttributeValueType),
}

impl TryFrom<Pair<'_, Rule>> for AttributeDefinition {
    type Error = DbcError;

    /// Parse attribute definition: `BA_DEF_ [object_type] attribute_name attribute_type [min max];`
    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let rule = pair.as_rule();
        let expected = match rule {
            Rule::attr_def => Rule::object_type,
            Rule::ba_def_rel => Rule::rel_object_type,
            v => Err(DbcError::ExpectedOneOfRules(
                vec![Rule::attr_def, Rule::ba_def_rel],
                v,
            ))?,
        };
        let mut pairs = pair.into_inner();
        let object_type = if let Some(v) = next_optional_rule(&mut pairs, expected) {
            v.as_str().to_string()
        } else {
            String::new()
        };

        let name = inner_str(next_rule(&mut pairs, Rule::attribute_name)?);
        let value = next(&mut pairs)?.try_into()?;
        expect_empty(&pairs)?;

        Ok(match object_type.as_str() {
            "SG_" | "BU_SG_REL_" => Self::Signal(name, value),
            "BO_" | "BU_BO_REL_" => Self::Message(name, value),
            "BU_" => Self::Node(name, value),
            "EV_" | "BU_EV_REL_" => Self::EnvironmentVariable(name, value),
            _ => Self::Plain(name, value),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{AttributeValueType, NumericValue};
    use crate::test_helpers::*;

    #[test]
    fn attribute_definition_bo_test() {
        let def = r#"
BA_DEF_ BO_ "BaDef1BO" INT 0 1000000;
"#;
        let exp = AttributeDefinition::Message(
            "BaDef1BO".to_string(),
            AttributeValueType::Int(NumericValue::Uint(0), NumericValue::Uint(1_000_000)),
        );
        let val = test_into::<AttributeDefinition>(def.trim_start(), Rule::attr_def);
        assert_eq!(val, exp);
    }

    #[test]
    fn attribute_definition_bo_txt_test() {
        let def = r#"
BA_DEF_ BO_  "GenMsgSendType" STRING ;
"#;
        let exp =
            AttributeDefinition::Message("GenMsgSendType".to_string(), AttributeValueType::String);
        let val = test_into::<AttributeDefinition>(def.trim_start(), Rule::attr_def);
        assert_eq!(val, exp);
    }

    #[test]
    fn attribute_definition_bu_test() {
        let def = r#"
BA_DEF_ BU_ "BuDef1BO" INT 0 1000000;
"#;
        let exp = AttributeDefinition::Node(
            "BuDef1BO".to_string(),
            AttributeValueType::Int(NumericValue::Uint(0), NumericValue::Uint(1_000_000)),
        );
        let val = test_into::<AttributeDefinition>(def.trim_start(), Rule::attr_def);
        assert_eq!(val, exp);
    }

    #[test]
    fn attribute_definition_sg_test() {
        let def = r#"
BA_DEF_ SG_ "SgDef1BO" INT 0 1000000;
"#;
        let exp = AttributeDefinition::Signal(
            "SgDef1BO".to_string(),
            AttributeValueType::Int(NumericValue::Uint(0), NumericValue::Uint(1_000_000)),
        );
        let val = test_into::<AttributeDefinition>(def.trim_start(), Rule::attr_def);
        assert_eq!(val, exp);
    }

    #[test]
    fn attribute_definition_ev_test() {
        let def = r#"
BA_DEF_ EV_ "EvDef1BO" INT 0 1000000;
"#;
        let exp = AttributeDefinition::EnvironmentVariable(
            "EvDef1BO".to_string(),
            AttributeValueType::Int(NumericValue::Uint(0), NumericValue::Uint(1_000_000)),
        );
        let val = test_into::<AttributeDefinition>(def.trim_start(), Rule::attr_def);
        assert_eq!(val, exp);
    }
}
