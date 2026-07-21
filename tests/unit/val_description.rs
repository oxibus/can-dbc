//! Unit tests for `val_description`.

use can_dbc::ValDescription;
use can_dbc_pest::Rule;

use crate::common::test_into;

#[test]
fn value_description_test() {
    let def = r#"
2 "ABC"
"#;
    let exp = ValDescription {
        id: 2,
        description: "ABC".to_string(),
    };
    let val = test_into::<ValDescription>(def.trim_start(), Rule::table_value_description);
    assert_eq!(val, exp);
}
