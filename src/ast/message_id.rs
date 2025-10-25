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
    pub(crate) fn parse(msg_id: u32) -> Self {
        if msg_id & (1 << 31) != 0 {
            Self::Extended(msg_id & 0x1FFF_FFFF)
        } else {
            Self::Standard(msg_id as u16)
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
