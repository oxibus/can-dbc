//! Unit tests for `value_description`.

use can_dbc::{MessageId, ValDescription, ValueDescription};
use can_dbc_pest::Rule;

use crate::common::test_into;

#[test]
fn value_description_for_signal_test() {
    let def = r#"
VAL_ 837 UF_HZ_OI 255 "NOP";
"#;
    let exp = ValueDescription::Signal {
        message_id: MessageId::Standard(837),
        name: "UF_HZ_OI".to_string(),
        value_descriptions: vec![ValDescription {
            id: 255,
            description: "NOP".to_string(),
        }],
    };
    let val = test_into::<ValueDescription>(def.trim_start(), Rule::value_table_def);
    assert_eq!(val, exp);
}

#[test]
fn value_description_for_env_var_test() {
    let def = r#"
VAL_ MY_ENV_VAR 255 "NOP";
"#;
    let exp = ValueDescription::EnvironmentVariable {
        name: "MY_ENV_VAR".to_string(),
        value_descriptions: vec![ValDescription {
            id: 255,
            description: "NOP".to_string(),
        }],
    };
    let val = test_into::<ValueDescription>(def.trim_start(), Rule::value_table_def);
    assert_eq!(val, exp);
}
