use crate::ast::MessageId;

/// Signal groups define a group of signals within a message
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SignalGroups {
    pub message_id: MessageId,
    pub name: String,
    pub repetitions: u64,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub signal_names: Vec<String>,
}
