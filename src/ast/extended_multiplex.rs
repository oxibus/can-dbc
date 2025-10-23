use crate::ast::{ExtendedMultiplexMapping, MessageId};

/// Mapping between multiplexors and multiplexed signals
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendedMultiplex {
    pub message_id: MessageId,
    pub signal_name: String,
    pub multiplexor_signal_name: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub mappings: Vec<ExtendedMultiplexMapping>,
}
