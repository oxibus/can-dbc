use crate::ast::AttributeValuedForObjectType;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttributeValueForObject {
    pub name: String,
    pub value: AttributeValuedForObjectType,
}
