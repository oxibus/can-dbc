use can_dbc_pest::{Pair, Rule};

use crate::ast::ValDescription;
use crate::parser::{next_rule, DbcResult};

/// Global value table
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValueTable {
    pub name: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub descriptions: Vec<ValDescription>,
}

impl ValueTable {
    /// Parse value table: `VAL_TABLE_ table_name value1 "description1" value2 "description2" ... ;`
    pub(crate) fn parse(pair: Pair<Rule>) -> DbcResult<ValueTable> {
        let mut pairs = pair.into_inner();

        let table_name = next_rule(&mut pairs, Rule::table_name)?
            .as_str()
            .to_string();

        // Collect table value descriptions
        let mut descriptions = Vec::new();
        for pair2 in pairs {
            if pair2.as_rule() == Rule::table_value_description {
                descriptions.push(ValDescription::parse_table_value_description(pair2)?);
            }
        }

        Ok(ValueTable {
            name: table_name,
            descriptions,
        })
    }
}
