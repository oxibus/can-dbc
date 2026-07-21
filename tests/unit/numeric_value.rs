//! Unit tests for `numeric_value`.

use std::str::FromStr;

use can_dbc::NumericValue;

#[test]
fn unsigned_max() {
    let value = u64::MAX.to_string();
    assert_eq!(
        format!("{}", NumericValue::from_str(&value).unwrap()),
        value
    );
}

#[test]
fn signed_min() {
    let value = i64::MIN.to_string();
    assert_eq!(
        format!("{}", NumericValue::from_str(&value).unwrap()),
        value
    );
}

#[test]
fn double() {
    let value = "3.141592653589793";
    assert_eq!(format!("{}", NumericValue::from_str(value).unwrap()), value);
}
