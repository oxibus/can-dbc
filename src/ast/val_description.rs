use can_dbc_pest::{Pair, Rule};

use crate::parser::{expect_empty, inner_str, next_rule, parse_int, validated_inner};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValDescription {
    pub id: i64,
    pub description: String,
}

impl TryFrom<Pair<'_, Rule>> for ValDescription {
    type Error = crate::parser::DbcError;

    fn try_from(value: Pair<Rule>) -> Result<Self, Self::Error> {
        let mut pairs = validated_inner(value, Rule::table_value_description)?;
        let id = parse_int(&next_rule(&mut pairs, Rule::int)?)?;
        let description = inner_str(next_rule(&mut pairs, Rule::quoted_str)?);
        expect_empty(&pairs)?;
        Ok(Self { id, description })
    }
}
