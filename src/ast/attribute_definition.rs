use can_dbc_pest::{Pair, Rule};

use crate::parser::{
    expect_empty, inner_str, next, next_optional_rule, next_rule, validated_inner, DbcError,
};
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
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut pairs = validated_inner(value, Rule::attr_def)?;

        let object_type = if let Some(v) = next_optional_rule(&mut pairs, Rule::object_type) {
            v.as_str().to_string()
        } else {
            String::new()
        };

        let name = inner_str(next_rule(&mut pairs, Rule::attribute_name)?);
        let value = next(&mut pairs)?.try_into()?;
        expect_empty(&pairs)?;

        Ok(match object_type.as_str() {
            "SG_" => Self::Signal(name, value),
            "BO_" => Self::Message(name, value),
            "BU_" => Self::Node(name, value),
            "EV_" => Self::EnvironmentVariable(name, value),
            _ => Self::Plain(name, value),
        })
    }
}
