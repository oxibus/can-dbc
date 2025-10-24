use can_dbc_pest::{Pair, Rule};

use crate::DbcResult;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Symbol(pub String);

impl Symbol {
    /// Parse new symbols: NS_ : symbol1 symbol2 ...
    pub(crate) fn parse_new_symbols(pair: Pair<Rule>) -> DbcResult<Vec<Symbol>> {
        let mut symbols = Vec::new();
        for pair2 in pair.into_inner() {
            if let Rule::ident = pair2.as_rule() {
                symbols.push(Symbol(pair2.as_str().to_string()));
            }
        }
        Ok(symbols)
    }
}
