use std::str::FromStr;

use crate::DbcError;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NumericValue {
    Uint(u64),
    Int(i64),
    Double(f64),
}

impl FromStr for NumericValue {
    type Err = DbcError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Ok(v) = value.parse::<u64>() {
            Ok(NumericValue::Uint(v))
        } else if let Ok(v) = value.parse::<i64>() {
            Ok(NumericValue::Int(v))
        } else if let Ok(v) = value.parse::<f64>() {
            Ok(NumericValue::Double(v))
        } else {
            Err(DbcError::InvalidNumericValue(value.to_string()))
        }
    }
}
