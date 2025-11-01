use can_dbc_pest::{Pair, Rule};

use crate::parser::{
    expect_empty, inner_str, next, next_optional_rule, next_rule, parse_float, parse_int,
    validated_inner, DbcError,
};
use crate::{AttributeValueType, DbcResult};

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
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut pairs = validated_inner(value, Rule::attr_def)?;
        let mut object_type = String::new();

        if let Some(value) = next_optional_rule(&mut pairs, Rule::object_type)? {
            object_type = value.as_str().to_string();
        }

        let attribute_name = inner_str(next_rule(&mut pairs, Rule::attribute_name)?);

        let value = next(&mut pairs)?;
        let rule = value.as_rule();
        let attr_value_type = match rule {
            Rule::attribute_type_int | Rule::attribute_type_hex => {
                let mut pairs = value.into_inner();
                let min = parse_int(next_rule(&mut pairs, Rule::minimum)?)?;
                let max = parse_int(next_rule(&mut pairs, Rule::maximum)?)?;
                if rule == Rule::attribute_type_int {
                    AttributeValueType::Int(min, max)
                } else {
                    AttributeValueType::Hex(min, max)
                }
            }
            Rule::attribute_type_float => {
                let mut pairs = value.into_inner();
                let min = parse_float(next_rule(&mut pairs, Rule::minimum)?)?;
                let max = parse_float(next_rule(&mut pairs, Rule::maximum)?)?;
                AttributeValueType::Float(min, max)
            }
            Rule::attribute_type_string => AttributeValueType::String,
            Rule::attribute_type_enum => {
                let enum_values: DbcResult<_> = value
                    .into_inner()
                    .map(|pair| {
                        if pair.as_rule() == Rule::quoted_str {
                            Ok(inner_str(pair))
                        } else {
                            Err(DbcError::Expected(Rule::quoted_str, pair.as_rule()))
                        }
                    })
                    .collect();
                AttributeValueType::Enum(enum_values?)
            }
            v => return Err(DbcError::UnknownRule(v)),
        };

        expect_empty(&pairs)?;

        Ok(match object_type.as_str() {
            "SG_" => Self::Signal(attribute_name, attr_value_type),
            "BO_" => Self::Message(attribute_name, attr_value_type),
            "BU_" => Self::Node(attribute_name, attr_value_type),
            "EV_" => Self::EnvironmentVariable(attribute_name, attr_value_type),
            _ => Self::Plain(attribute_name, attr_value_type),
        })
    }
}
