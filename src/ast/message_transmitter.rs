use derive_getters::Getters;

use crate::ast::{MessageId, Transmitter};

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageTransmitter {
    pub message_id: MessageId,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub transmitter: Vec<Transmitter>,
}
