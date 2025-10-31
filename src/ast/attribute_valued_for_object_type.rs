use crate::ast::{AttributeValue, MessageId};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AttributeValuedForObjectType {
    Raw(AttributeValue),
    NetworkNode(String, AttributeValue),
    MessageDefinition(MessageId, Option<AttributeValue>),
    Signal(MessageId, String, AttributeValue),
    EnvVariable(String, AttributeValue),
}
