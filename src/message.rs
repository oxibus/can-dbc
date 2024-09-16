#[cfg(feature = "with-serde")]
extern crate serde;
#[cfg(feature = "with-serde")]
#[macro_use]
extern crate serde_derive;

use derive_getters::Getters;

use crate::DBCString;
use crate::nodes::Transmitter;
use crate::signal::Signal;

/// CAN id in header of CAN frame.
/// Must be unique in DBC file.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum MessageId {
    Standard(u16),
    /// 29 bit extended identifier without the extended bit.
    /// For the raw value of the message id including the bit for extended identifiers use the `raw()` method.
    Extended(u32),
}

impl MessageId {
    /// Raw value of the message id including the bit for extended identifiers
    pub fn raw(&self) -> u32 {
        match self {
            MessageId::Standard(id) => *id as u32,
            MessageId::Extended(id) => *id | 1 << 31,
        }
    }
}

impl DBCString for MessageId {
    fn dbc_string(&self) -> String {
        return match self {
            Self::Standard(id) => id.to_string(),
            Self::Extended(id) => id.to_string(),
        }
    }
}


/// CAN message (frame) details including signal details
#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Message {
    /// CAN id in header of CAN frame.
    /// Must be unique in DBC file.
    pub (crate) message_id: MessageId,
    pub (crate) message_name: String,
    pub (crate) message_size: u64,
    pub (crate) transmitter: Transmitter,
    pub (crate)  signals: Vec<Signal>,
}

impl DBCString for Message {
    fn dbc_string(&self) -> String {
        return format!("BO_ {} {}: {} {}\n  {}",
            self.message_id.dbc_string(),
            self.message_name,
            self.message_size,
            self.transmitter.dbc_string(),
            self.signals
                .clone()
                .into_iter()
                .map(|sg| sg.dbc_string())
                .collect::<Vec<String>>()
                .join("\n  ")
        )
    }
}
