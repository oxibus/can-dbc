use can_dbc_pest::{Pair, Rule};

use crate::parser::{inner_str, parse_float, DbcError};

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

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::quoted_str => Ok(Self::String(inner_str(pair))),
            Rule::number => Ok(Self::Double(parse_float(pair)?)),
            // FIXME: Add u64 and i64 parsing
            _ => Err(Self::Error::ParseError),
        }
    }
}
