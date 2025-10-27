use can_dbc_pest::{Pair, Rule};

use crate::ast::{AccessNode, AccessType, EnvType};
use crate::parser::{
    inner_str, parse_int, parse_min_max_int, single_inner_str, validated_inner, DbcError,
};

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

impl TryFrom<Pair<'_, Rule>> for EnvironmentVariable {
    type Error = DbcError;

    /// Parse environment variable: `EV_ variable_name : type [min|max] "unit" access_type access_node node_name1 node_name2;`
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let inner_pairs = validated_inner(value, Rule::environment_variable)?;

        let mut name = String::new();
        let mut env_type = None;
        let mut min = 0i64;
        let mut max = 0i64;
        let mut unit = String::new();
        let mut initial_value = 0.0f64;
        let mut ev_id = 0i64;
        let mut access_type = AccessType::DummyNodeVector0;
        let mut access_nodes = Vec::new();

        for pair in inner_pairs {
            match pair.as_rule() {
                Rule::env_var => name = single_inner_str(pair, Rule::env_var_name)?,
                Rule::env_var_type_int => env_type = Some(EnvType::Integer),
                Rule::env_var_type_float => env_type = Some(EnvType::Float),
                Rule::env_var_type_string => env_type = Some(EnvType::String),
                Rule::min_max => (min, max) = parse_min_max_int(pair)?,
                Rule::unit => unit = inner_str(pair),
                Rule::init_value => initial_value = parse_int(pair)? as f64,
                Rule::ev_id => ev_id = parse_int(pair)?,
                Rule::access_type => access_type = pair.try_into()?,
                Rule::node_name => access_nodes.push(pair.try_into()?),
                _ => panic!("Unexpected rule: {pair:?}"),
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
