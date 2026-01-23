use can_dbc_pest::{Pair, Rule};

use crate::parser::{parse_uint, single_inner, validated};
use crate::DbcError;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AccessType {
    DummyNodeVector0,
    DummyNodeVector1,
    DummyNodeVector2,
    DummyNodeVector3,
}

impl TryFrom<Pair<'_, Rule>> for AccessType {
    type Error = DbcError;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let pair = validated(value, Rule::access_type)?;
        let value = parse_uint(&single_inner(pair, Rule::uint)?)?;

        Ok(match value {
            0 => Self::DummyNodeVector0,
            1 => Self::DummyNodeVector1,
            2 => Self::DummyNodeVector2,
            3 => Self::DummyNodeVector3,
            // FIXME: is this correct?
            _ => AccessType::DummyNodeVector0,
        })
    }
}
