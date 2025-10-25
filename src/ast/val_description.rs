use can_dbc_pest::{Pair, Rule};

use crate::parser::{expect_empty, next_rule, parse_int, parse_str, DbcResult};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValDescription {
    pub id: f64,
    pub description: String,
}

impl ValDescription {
    /// Helper to parse a single table value description pair (value + description)
    pub(crate) fn parse_table_value_description(pair: Pair<Rule>) -> DbcResult<ValDescription> {
        pair.try_into()
    }
}

impl TryFrom<Pair<'_, Rule>> for ValDescription {
    type Error = crate::parser::DbcError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut pairs = pair.into_inner();
        let id = parse_int(next_rule(&mut pairs, Rule::int)?)? as f64;
        let description = parse_str(next_rule(&mut pairs, Rule::quoted_str)?);
        expect_empty(&mut pairs)?;
        Ok(ValDescription { id, description })
    }
}
