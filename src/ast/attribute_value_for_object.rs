use can_dbc_pest::{Pair, Rule};

use crate::ast::AttributeValuedForObjectType;
use crate::parser::{parse_next_inner_str, validated_inner, DbcError};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttributeValueForObject {
    pub name: String,
    pub value: AttributeValuedForObjectType,
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
