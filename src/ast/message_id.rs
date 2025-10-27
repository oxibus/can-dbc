use can_dbc_pest::{Pair, Rule};

use crate::parser::parse_uint;
use crate::DbcError;

/// CAN id in header of CAN frame.
/// Must be unique in DBC file.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MessageId {
    Standard(u16),
    /// 29 bit extended identifier without the extended bit.
    /// For the raw value of the message id including the bit for extended identifiers use the `raw()` method.
    Extended(u32),
}

impl MessageId {
    /// Raw value of the message id including the bit for extended identifiers
    pub fn raw(self) -> u32 {
        match self {
            Self::Standard(id) => u32::from(id),
            Self::Extended(id) => id | 1 << 31,
        }
    }
}

impl TryFrom<u64> for MessageId {
    type Error = DbcError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let value = u32::try_from(value).map_err(|_| DbcError::ParseError)?;
        if value & (1 << 31) != 0 {
            Ok(Self::Extended(value & 0x1FFF_FFFF))
        } else {
            Ok(Self::Standard(value as u16))
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for MessageId {
    type Error = DbcError;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        Self::try_from(parse_uint(value)?)
    }
}
