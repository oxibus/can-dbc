use crate::ast::{AccessNode, AccessType, EnvType};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnvironmentVariable {
    pub name: String,
    pub typ: EnvType,
    pub min: i64,
    pub max: i64,
    pub unit: String,
    pub initial_value: f64,
    pub ev_id: i64,
    pub access_type: AccessType,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub access_nodes: Vec<AccessNode>,
}
