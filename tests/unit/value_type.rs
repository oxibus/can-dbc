//! Unit tests for `value_type`.

use can_dbc::ValueType;
use can_dbc_pest::Rule;

use crate::common::test_into;

#[test]
fn value_type_test() {
    let val = test_into::<ValueType>("-", Rule::signed_type);
    assert_eq!(val, ValueType::Signed);

    let val = test_into::<ValueType>("+", Rule::unsigned_type);
    assert_eq!(val, ValueType::Unsigned);
}
