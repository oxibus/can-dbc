use can_dbc_pest::{Pair, Rule};

use crate::ast::ValDescription;
use crate::parser::{collect_expected, next_string, validated_inner, DbcError};

/// Global value table
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValueTable {
    pub name: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub descriptions: Vec<ValDescription>,
}

impl TryFrom<Pair<'_, Rule>> for ValueTable {
    type Error = DbcError;

    /// Parse value table: `VAL_TABLE_ table_name value1 "description1" value2 "description2" ... ;`
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut pairs = validated_inner(value, Rule::value_table)?;

        let name = next_string(&mut pairs, Rule::table_name)?;
        let descriptions = collect_expected(&mut pairs, Rule::table_value_description)?;

        Ok(Self { name, descriptions })
    }
}
