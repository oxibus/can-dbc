use derive_getters::Getters;

use crate::ast::MessageId;

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SignalTypeRef {
    pub message_id: MessageId,
    pub signal_name: String,
    pub signal_type_name: String,
}
