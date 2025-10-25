use can_dbc_pest::{Pair, Rule};

use crate::parser::{collect_strings, DbcResult};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Symbol(pub String);

impl Symbol {
    /// Parse new symbols: NS_ : symbol1 symbol2 ...
    pub(crate) fn parse(pair: Pair<Rule>) -> DbcResult<Vec<Symbol>> {
        let mut pairs = pair.into_inner();
        let symbol_names = collect_strings(&mut pairs, Rule::ident)?;
        Ok(symbol_names.into_iter().map(Symbol).collect())
    }
}
