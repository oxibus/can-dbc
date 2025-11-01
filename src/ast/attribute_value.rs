use can_dbc_pest::{Pair, Rule};

use crate::parser::{inner_str, parse_float, parse_int, DbcError};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AttributeValue {
    U64(u64),
    I64(i64),
    Double(f64),
    String(String),
}

impl TryFrom<Pair<'_, Rule>> for AttributeValue {
    type Error = DbcError;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match value.as_rule() {
            Rule::quoted_str => Ok(Self::String(inner_str(value))),
            Rule::number => {
                if let Ok(result) = parse_int(&value) {
                    Ok(Self::I64(result))
                } else {
                    Ok(Self::Double(parse_float(&value)?))
                }
            }
            _ => Err(Self::Error::ExpectedStrNumber(value.as_rule())),
        }
    }
}
