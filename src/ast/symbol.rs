use can_dbc_pest::{Pair, Rule};

use crate::parser::{validated, DbcError};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Symbol(pub String);

impl TryFrom<Pair<'_, Rule>> for Symbol {
    type Error = DbcError;

    /// Parse new symbols: NS_ : symbol1 symbol2 ...
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        Ok(Self(validated(value, Rule::ident)?.as_str().to_string()))
    }
}
