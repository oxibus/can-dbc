use can_dbc_pest::{Pair, Rule};

use crate::parser::DbcResult;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Symbol(pub String);

impl Symbol {
    /// Parse new symbols: NS_ : symbol1 symbol2 ...
    pub(crate) fn parse_new_symbols(pair: Pair<Rule>) -> DbcResult<Vec<Symbol>> {
        Ok(pair
            .into_inner()
            .filter(|pair| pair.as_rule() == Rule::ident)
            .map(|pair| Symbol(pair.as_str().to_string()))
            .collect())
    }
}
