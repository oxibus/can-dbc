use crate::ast::{MessageId, ValDescription};

/// Encoding for signal raw values.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ValueDescription {
    Signal {
        message_id: MessageId,
        name: String,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
        value_descriptions: Vec<ValDescription>,
    },
    EnvironmentVariable {
        name: String,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
        value_descriptions: Vec<ValDescription>,
    },
}
