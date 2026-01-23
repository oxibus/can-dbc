use can_dbc_pest::{Pair, Rule};

use crate::parser::{expect_empty, parse_next_uint, validated_inner, DbcError};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendedMultiplexMapping {
    pub min_value: u64,
    pub max_value: u64,
}

impl TryFrom<Pair<'_, Rule>> for ExtendedMultiplexMapping {
    type Error = DbcError;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut pairs = validated_inner(value, Rule::value_pair)?;
        let value = ExtendedMultiplexMapping {
            min_value: parse_next_uint(&mut pairs, Rule::uint)?,
            max_value: parse_next_uint(&mut pairs, Rule::uint)?,
        };
        expect_empty(&pairs)?;
        Ok(value)
    }
}
