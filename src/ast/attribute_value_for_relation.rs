use can_dbc_pest::{Pair, Rule};

use crate::parser::{next, parse_next_inner_str, validated_inner, DbcError};
use crate::AttributeValueForRelationType;

/// An attribute attached to the relation between a node and a signal
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttributeValueForRelation {
    pub name: String,
    pub details: AttributeValueForRelationType,
}

impl TryFrom<Pair<'_, Rule>> for AttributeValueForRelation {
    type Error = DbcError;

    /// Parse attribute value for relation: `BA_REL_ attribute_name rel_object_type rel_object_data;`
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut pairs = validated_inner(value, Rule::ba_rel)?;

        let name = parse_next_inner_str(&mut pairs, Rule::attribute_name)?;
        let _rel_object_type = next(&mut pairs)?; // We'll use it in rel_object_data parsing
        let rel_object_data = next(&mut pairs)?;

        Ok(Self {
            name,
            details: rel_object_data.try_into()?,
        })
    }
}
