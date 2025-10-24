use can_dbc_pest::{Pair, Rule};

use crate::parser;
use crate::parser::DbcResult;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValDescription {
    pub id: f64,
    pub description: String,
}

impl ValDescription {
    /// Helper to parse a single table value description pair (value + description)
    pub(crate) fn parse_table_value_description(pair: Pair<Rule>) -> DbcResult<ValDescription> {
        let mut pairs = pair.into_inner();

        let id = parser::parse_int(parser::next_rule(&mut pairs, Rule::int)?)? as f64;
        let description = parser::parse_str(parser::next_rule(&mut pairs, Rule::quoted_str)?);
        // Don't use expect_empty here as there might be comments or whitespace

        Ok(ValDescription { id, description })
    }
}
