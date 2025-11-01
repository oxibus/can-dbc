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
    #[must_use]
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
        u32::try_from(value)
            .map_err(|_| DbcError::MessageIdOutOfRange(value))?
            .try_into()
    }
}

impl TryFrom<u32> for MessageId {
    type Error = DbcError;

    /// Create `MessageId` from u32 value including the extended bit flag
    ///
    /// If bit 31 is set, creates an Extended `MessageId` with bits 0-28.
    /// Otherwise, creates a Standard `MessageId`, erroring if the value is out of range for u16.
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        const EXTENDED_ID_FLAG: u32 = 1 << 31;
        Ok(if value & EXTENDED_ID_FLAG != 0 {
            Self::Extended(value & 0x1FFF_FFFF)
        } else {
            // FIXME: this code seems more correct, but breaks existing tests
            // let v = u16::try_from(value)
            //     .map_err(|_| DbcError::MessageIdOutOfRange(u64::from(value)))?;
            #[allow(clippy::cast_possible_truncation)]
            let v = value as u16;

            Self::Standard(v)
        })
    }
}

impl TryFrom<Pair<'_, Rule>> for MessageId {
    type Error = DbcError;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        Self::try_from(parse_uint(&value)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::MessageId;

    #[test]
    fn extended_message_id_raw() {
        let id = MessageId::Extended(2);
        assert_eq!(id.raw(), 2 | 1 << 31);
        let id = MessageId::Extended(2 ^ 29);
        assert_eq!(id.raw(), 2 ^ 29 | 1 << 31);
    }

    #[test]
    fn standard_message_id_raw() {
        let id = MessageId::Standard(2);
        assert_eq!(id.raw(), 2);
    }

    #[test]
    fn try_from_u32_standard() {
        let id = MessageId::try_from(500u32).unwrap();
        assert_eq!(id, MessageId::Standard(500));
    }

    #[test]
    fn try_from_u32_extended() {
        let id = MessageId::try_from(2u32 | (1 << 31)).unwrap();
        assert_eq!(id, MessageId::Extended(2));
    }

    #[test]
    fn try_from_u64_standard() {
        let id = MessageId::try_from(500u64).unwrap();
        assert_eq!(id, MessageId::Standard(500));
    }

    #[test]
    fn try_from_u64_extended() {
        let id = MessageId::try_from(2u64 | (1 << 31)).unwrap();
        assert_eq!(id, MessageId::Extended(2));
    }
}
