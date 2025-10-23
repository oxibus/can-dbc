use derive_getters::Getters;

use crate::ast::AttributeValue;

// FIXME: not used!
#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttrDefault {
    name: String,
    value: AttributeValue,
}
