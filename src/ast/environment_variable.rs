use can_dbc_pest::{Pair, Rule};

use crate::ast::{AccessNode, AccessType, EnvType};
use crate::parser::{
    collect_expected, expect_empty, inner_str, next, next_optional_rule, next_rule, parse_int,
    parse_min_max_int, single_inner_str, validated_inner, DbcError,
};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnvironmentVariable {
    pub name: String,
    pub typ: EnvType,
    pub min: i64,
    pub max: i64,
    pub unit: String,
    pub initial_value: i64,
    pub ev_id: i64,
    pub access_type: AccessType,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub access_nodes: Vec<AccessNode>,
}

impl TryFrom<Pair<'_, Rule>> for EnvironmentVariable {
    type Error = DbcError;

    /// Parse environment variable: `EV_ variable_name : type [min|max] "unit" access_type access_node node_name1 node_name2;`
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        // 1) Validate wrapper and get iterator
        let mut pairs = validated_inner(value, Rule::environment_variable)?;

        // 2) Required: env_var (wrapper containing env_var_name)
        let name = single_inner_str(next_rule(&mut pairs, Rule::env_var)?, Rule::env_var_name)?;

        // 3) Required: env var type (one of three rules)
        let typ = next(&mut pairs)?.as_rule().try_into()?;

        // 4) Optional: min_max
        let (mut min, mut max) = (0i64, 0i64);
        if let Some(min_max_pair) = next_optional_rule(&mut pairs, Rule::min_max) {
            (min, max) = parse_min_max_int(min_max_pair)?;
        }

        // 5) Optional: unit
        let mut unit = String::new();
        if let Some(unit_pair) = next_optional_rule(&mut pairs, Rule::unit) {
            unit = inner_str(unit_pair);
        }

        // 6) Optional: init_value
        let mut initial_value = 0;
        if let Some(init_pair) = next_optional_rule(&mut pairs, Rule::init_value) {
            initial_value = parse_int(&init_pair)?;
        }

        // 7) Optional: ev_id
        let mut ev_id = 0i64;
        if let Some(ev_pair) = next_optional_rule(&mut pairs, Rule::ev_id) {
            ev_id = parse_int(&ev_pair)?;
        }

        // 8) Required: access_type
        let access_type = next_rule(&mut pairs, Rule::access_type)?.try_into()?;

        // 9) Remaining: zero or more node_name entries -> collect into AccessNode
        let access_nodes = collect_expected::<AccessNode>(&mut pairs, Rule::node_name)?;

        expect_empty(&pairs)?;

        Ok(Self {
            name,
            typ,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

    #[test]
    fn environment_variable_test() {
        let def = r#"
EV_ IUV: 0 [-22|20] "mm" 3 7 DUMMY_NODE_VECTOR0 VECTOR_XXX;
"#;
        let exp = EnvironmentVariable {
            name: "IUV".to_string(),
            typ: EnvType::Integer,
            min: -22,
            max: 20,
            unit: "mm".to_string(),
            initial_value: 3,
            ev_id: 7,
            access_type: AccessType::DummyNodeVector0,
            access_nodes: vec![AccessNode::Name("VECTOR_XXX".to_string())],
        };
        let val = test_into::<EnvironmentVariable>(def.trim_start(), Rule::environment_variable);
        assert_eq!(val, exp);
    }
}
