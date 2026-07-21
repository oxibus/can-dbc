//! Unit tests for `value_table`.

use can_dbc::{ValDescription, ValueTable};
use can_dbc_pest::Rule;

use crate::common::test_into;

#[test]
fn val_table_test() {
    let def = r#"
VAL_TABLE_ Tst 2 "ABC" 1 "Test A" ;
"#;
    let exp = ValueTable {
        name: "Tst".to_string(),
        descriptions: vec![
            ValDescription {
                id: 2,
                description: "ABC".to_string(),
            },
            ValDescription {
                id: 1,
                description: "Test A".to_string(),
            },
        ],
    };
    let val = test_into::<ValueTable>(def.trim_start(), Rule::value_table);
    assert_eq!(val, exp);
}

#[test]
fn val_table_no_space_preceding_comma_test() {
    let def = r#"
VAL_TABLE_ Tst 2 "ABC";
"#;
    let exp = ValueTable {
        name: "Tst".to_string(),
        descriptions: vec![ValDescription {
            id: 2,
            description: "ABC".to_string(),
        }],
    };
    let val = test_into::<ValueTable>(def.trim_start(), Rule::value_table);
    assert_eq!(val, exp);
}
