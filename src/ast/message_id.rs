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
    /// Create MessageId from raw value including the extended bit flag
    ///
    /// If bit 31 is set, creates an Extended MessageId with bits 0-28.
    /// Otherwise, creates a Standard MessageId.
    pub fn from_raw(raw_id: u32) -> Self {
        const EXTENDED_ID_FLAG: u32 = 1 << 31;
        const EXTENDED_ID_MASK: u32 = 0x1FFF_FFFF;

        if raw_id & EXTENDED_ID_FLAG != 0 {
            Self::Extended(raw_id & EXTENDED_ID_MASK)
        } else {
            Self::Standard(raw_id as u16)
        }
    }

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
        let value = u32::try_from(value).map_err(|_| DbcError::MessageIdOutOfRange(value))?;
        Ok(Self::from_raw(value))
    }
}

impl TryFrom<Pair<'_, Rule>> for MessageId {
    type Error = DbcError;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        Self::try_from(parse_uint(value)?)
    }
}
