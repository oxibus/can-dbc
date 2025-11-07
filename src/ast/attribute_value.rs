use can_dbc_pest::{Pair, Rule};

use crate::parser::{inner_str, DbcError};
use crate::NumericValue;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AttributeValue {
    Uint(u64),
    Int(i64),
    Double(f64),
    String(String),
}

impl TryFrom<Pair<'_, Rule>> for AttributeValue {
    type Error = DbcError;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match value.as_rule() {
            Rule::quoted_str => Ok(Self::String(inner_str(value))),
            Rule::number => Ok(match value.as_str().parse()? {
                NumericValue::Uint(u) => Self::Uint(u),
                NumericValue::Int(i) => Self::Int(i),
                NumericValue::Double(d) => Self::Double(d),
            }),
            _ => Err(Self::Error::ExpectedStrNumber(value.as_rule())),
        }
    }
}
