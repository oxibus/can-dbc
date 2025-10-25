use can_dbc_pest::{Pair, Rule};

use crate::ast::{AccessNode, AccessType, EnvType};
use crate::parser::{parse_int, parse_min_max_int, parse_str, single_string, DbcResult};

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
    pub(crate) fn parse(pair: Pair<Rule>) -> DbcResult<Self> {
        let mut name = String::new();
        let mut env_type = None;
        let mut min = 0i64;
        let mut max = 0i64;
        let mut unit = String::new();
        let mut initial_value = 0.0f64;
        let mut ev_id = 0i64;
        let mut access_type = AccessType::DummyNodeVector0;
        let mut access_nodes = Vec::new();

        for pairs in pair.into_inner() {
            match pairs.as_rule() {
                Rule::env_var => name = single_string(pairs, Rule::env_var_name)?,
                Rule::env_var_type_int => env_type = Some(EnvType::Integer),
                Rule::env_var_type_float => env_type = Some(EnvType::Float),
                Rule::env_var_type_string => env_type = Some(EnvType::String),
                Rule::min_max => (min, max) = parse_min_max_int(pairs)?,
                Rule::unit => unit = parse_str(pairs),
                Rule::init_value => initial_value = parse_int(pairs)? as f64,
                Rule::ev_id => ev_id = parse_int(pairs)?,
                Rule::access_type => access_type = pairs.try_into()?,
                Rule::node_name => access_nodes.push(pairs.try_into()?),
                _ => panic!("Unexpected rule: {pairs:?}"),
            }
        }

        Ok(Self {
            name,
            typ: env_type.unwrap(),
            min,
            max,
            unit,
            initial_value,
            ev_id,
            access_type,
            access_nodes,
        })
    }
}
