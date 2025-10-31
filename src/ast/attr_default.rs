use crate::ast::AttributeValue;

// FIXME: not used!
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttrDefault {
    pub name: String,
    pub value: AttributeValue,
}
