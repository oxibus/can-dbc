//! Unit tests for `attribute_default`.

use can_dbc::{AttributeDefault, AttributeValue};
use can_dbc_pest::Rule;

use crate::common::test_into;

#[test]
fn attribute_default_test() {
    let def = r#"
BA_DEF_DEF_  "ZUV" "OAL";
"#;
    let exp = AttributeDefault {
        name: "ZUV".to_string(),
        value: AttributeValue::String("OAL".to_string()),
    };
    let val = test_into::<AttributeDefault>(def.trim_start(), Rule::ba_def_def);
    assert_eq!(val, exp);
}
