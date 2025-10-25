use can_dbc_pest::{Pair, Rule};

use crate::parser::{parse_uint, single_rule};
use crate::DbcError;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AccessType {
    DummyNodeVector0,
    DummyNodeVector1,
    DummyNodeVector2,
    DummyNodeVector3,
}

impl TryFrom<Pair<'_, Rule>> for AccessType {
    type Error = DbcError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::access_type => {
                match parse_uint(single_rule(pair, Rule::uint)?)? {
                    0 => Ok(Self::DummyNodeVector0),
                    1 => Ok(Self::DummyNodeVector1),
                    2 => Ok(Self::DummyNodeVector2),
                    3 => Ok(Self::DummyNodeVector3),
                    // FIXME: is this correct?
                    _ => Ok(AccessType::DummyNodeVector0),
                }
            }
            _ => Err(Self::Error::ParseError),
        }
    }
}
