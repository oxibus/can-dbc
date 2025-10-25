use can_dbc_pest::{Pair, Rule};

use crate::DbcError;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}

impl TryFrom<Pair<'_, Rule>> for ByteOrder {
    type Error = DbcError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::little_endian => Ok(Self::LittleEndian),
            Rule::big_endian => Ok(Self::BigEndian),
            _ => Err(Self::Error::ParseError),
        }
    }
}
