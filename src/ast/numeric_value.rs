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

impl std::fmt::Display for NumericValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumericValue::Uint(v) => write!(f, "{v}"),
            NumericValue::Int(v) => write!(f, "{v}"),
            NumericValue::Double(v) => write!(f, "{v}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
