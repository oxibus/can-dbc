//! Unit tests for `environment_variable_data`.

use can_dbc::EnvironmentVariableData;
use can_dbc_pest::Rule;

use crate::common::test_into;

#[test]
fn envvar_data_test() {
    let def = "
ENVVAR_DATA_ SomeEnvVarData: 399;
";
    let exp = EnvironmentVariableData {
        env_var_name: "SomeEnvVarData".to_string(),
        data_size: 399,
    };
    let val = test_into::<EnvironmentVariableData>(def.trim_start(), Rule::env_var_data);
    assert_eq!(val, exp);
}
