use can_dbc_pest::{Pair, Rule};

use crate::DbcError;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ValueType {
    Signed,
    Unsigned,
}

impl TryFrom<Pair<'_, Rule>> for ValueType {
    type Error = DbcError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::signed_type => Ok(Self::Signed),
            Rule::unsigned_type => Ok(Self::Unsigned),
            _ => Err(Self::Error::ParseError),
        }
    }
}
