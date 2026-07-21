//! Unit tests for `environment_variable`.

use can_dbc::{AccessType, EnvType, EnvironmentVariable};
use can_dbc_pest::Rule;

use crate::common::test_into;

#[test]
fn environment_variable_test() {
    // NOTE: "VECTOR_XXX" is not treated in a special way,
    // unlike the "VECTOR__XXX".
    // This might be a bug, but it is consistent with python and other tooling

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
        access_nodes: vec!["VECTOR_XXX".to_string()],
    };
    let val = test_into::<EnvironmentVariable>(def.trim_start(), Rule::environment_variable);
    assert_eq!(val, exp);
}

#[test]
fn vector_placeholder_access_node_test() {
    let def = r#"
EV_ IUV: 0 [-22|20] "mm" 3 7 DUMMY_NODE_VECTOR0 Vector__XXX;
"#;
    let val = test_into::<EnvironmentVariable>(def.trim_start(), Rule::environment_variable);
    assert_eq!(val.access_nodes, Vec::<String>::new());
}
