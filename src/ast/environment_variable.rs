use can_dbc_pest::{Pair, Rule};

use crate::ast::{AccessNode, AccessType, EnvType};
use crate::parser::{parse_int, parse_min_max_int, parse_str, DbcResult};

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

impl EnvironmentVariable {
    /// Parse environment variable: `EV_ variable_name : type [min|max] "unit" access_type access_node node_name1 node_name2;`
    pub(crate) fn parse(pair: Pair<Rule>) -> DbcResult<EnvironmentVariable> {
        let mut variable_name = String::new();
        let mut env_type = 0u64;
        let mut min_val = 0i64;
        let mut max_val = 0i64;
        let mut unit = String::new();
        let mut initial_value = 0.0f64;
        let mut ev_id = 0i64;
        let mut access_type = String::new();
        let mut access_nodes = Vec::new();

        for pairs in pair.into_inner() {
            match pairs.as_rule() {
                Rule::env_var => {
                    // env_var contains env_literal ~ env_var_name
                    for sub_pair in pairs.into_inner() {
                        if sub_pair.as_rule() == Rule::env_var_name {
                            variable_name = sub_pair.as_str().to_string();
                        }
                    }
                }
                Rule::env_var_type_int => env_type = 0, // Integer type
                Rule::env_var_type_float => env_type = 1, // Float type
                Rule::env_var_type_string => env_type = 2, // String type
                Rule::min_max => (min_val, max_val) = parse_min_max_int(pairs)?,
                Rule::unit => unit = parse_str(pairs),
                Rule::init_value => initial_value = parse_int(pairs)? as f64,
                Rule::ev_id => ev_id = parse_int(pairs)?,
                Rule::node_name => {
                    let name = pairs.as_str().to_string();
                    if access_type.is_empty() {
                        // First node_name is the access type
                        access_type = name;
                    } else {
                        // Subsequent node_names are access nodes
                        let access_node = if name == "VECTOR__XXX" {
                            AccessNode::VectorXXX
                        } else {
                            AccessNode::Name(name)
                        };
                        access_nodes.push(access_node);
                    }
                }
                other => panic!("What is this? {other:?}"),
            }
        }

        let typ = match env_type {
            0 => EnvType::Float,
            1 => EnvType::U64,
            2 => EnvType::Data,
            v => panic!("Unknown environment variable type: {v}"),
        };

        let access_type_enum = match access_type.as_str() {
            "DUMMY_NODE_VECTOR1" => AccessType::DummyNodeVector1,
            "DUMMY_NODE_VECTOR2" => AccessType::DummyNodeVector2,
            "DUMMY_NODE_VECTOR3" => AccessType::DummyNodeVector3,
            // FIXME: is this correct?
            _ => AccessType::DummyNodeVector0,
        };

        Ok(EnvironmentVariable {
            name: variable_name,
            typ,
            min: min_val,
            max: max_val,
            unit,
            initial_value,
            ev_id,
            access_type: access_type_enum,
            access_nodes,
        })
    }
}
