use can_dbc_pest::{Pair, Rule};

use crate::ast::{val_description, ValDescription};
use crate::{parser, DbcResult};

/// Global value table
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValueTable {
    pub name: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub descriptions: Vec<ValDescription>,
}

/// Parse value table: `VAL_TABLE_ table_name value1 "description1" value2 "description2" ... ;`
pub(crate) fn parse_value_table(pair: Pair<Rule>) -> DbcResult<ValueTable> {
    let mut inner_pairs = pair.into_inner();

    let table_name = parser::next_rule(&mut inner_pairs, Rule::table_name)?
        .as_str()
        .to_string();

    // Collect table value descriptions
    let mut descriptions = Vec::new();
    for pair2 in inner_pairs {
        if pair2.as_rule() == Rule::table_value_description {
            descriptions.push(val_description::parse_table_value_description(pair2)?);
        }
    }

    Ok(ValueTable {
        name: table_name,
        descriptions,
    })
}
