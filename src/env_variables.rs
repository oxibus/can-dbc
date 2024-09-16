#[cfg(feature = "with-serde")]
extern crate serde;
#[cfg(feature = "with-serde")]
#[macro_use]
extern crate serde_derive;

use derive_getters::Getters;

use crate::DBCString;
use crate::nodes::{AccessType, AccessNode};

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct EnvironmentVariable {
    pub (crate) env_var_name: String,
    pub (crate) env_var_type: EnvType,
    pub (crate) min: i64,
    pub (crate) max: i64,
    pub (crate) unit: String,
    pub (crate) initial_value: f64,
    pub (crate) ev_id: i64,
    pub (crate) access_type: AccessType,
    pub (crate) access_nodes: Vec<AccessNode>,
}

impl DBCString for EnvironmentVariable {
    fn dbc_string(&self) -> String {
        return format!("EV_ {}: {} [{}|{}] \"{}\" {} {} {} {};",
            self.env_var_name,
            self.env_var_type.dbc_string(),
            self.min,
            self.max,
            self.unit,
            self.initial_value,
            self.ev_id,
            self.access_type.dbc_string(),
            self.access_nodes
                .clone()
                .into_iter()
                .map(|an| an.dbc_string())
                .collect::<Vec<String>>()
                .join(";")
        )
    }
}

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct EnvironmentVariableData {
    pub (crate) env_var_name: String,
    pub (crate) data_size: u64,
}

impl DBCString for EnvironmentVariableData {
    fn dbc_string(&self) -> String {
        return format!("ENVVAR_DATA_ {}: {};",
            self.env_var_name,
            self.data_size,
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum EnvType {
    EnvTypeFloat,
    EnvTypeu64,
    EnvTypeData,
}

impl DBCString for EnvType {
    fn dbc_string(&self) -> String {
        return match self {
            Self::EnvTypeFloat => "0".to_string(),
            Self::EnvTypeu64 => "1".to_string(),
            Self::EnvTypeData => "".to_string(), // TODO determine what this value should enumerate to
        }
    }
}
