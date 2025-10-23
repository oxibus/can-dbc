use derive_getters::Getters;

use crate::ast::AttributeValuedForObjectType;

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttributeValueForObject {
    pub name: String,
    pub value: AttributeValuedForObjectType,
}
