use can_dbc_pest::{Pair, Rule};

use crate::parser::{expect_empty, next_rule, parse_uint, validated, DbcError};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendedMultiplexMapping {
    pub min_value: u64,
    pub max_value: u64,
}

impl TryFrom<Pair<'_, Rule>> for ExtendedMultiplexMapping {
    type Error = DbcError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut iter = validated(pair, Rule::value_pair)?.into_inner();
        let value = ExtendedMultiplexMapping {
            min_value: parse_uint(next_rule(&mut iter, Rule::uint)?)?,
            max_value: parse_uint(next_rule(&mut iter, Rule::uint)?)?,
        };
        expect_empty(&iter)?;
        Ok(value)
    }
}
