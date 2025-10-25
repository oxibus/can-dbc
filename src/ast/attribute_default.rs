use can_dbc_pest::{Pair, Rule};

use crate::ast::AttributeValue;
use crate::parser::{expect_empty, next_rule, parse_str, DbcError, DbcResult};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttributeDefault {
    pub name: String,
    pub value: AttributeValue,
}

impl AttributeDefault {
    /// Parse attribute default: `BA_DEF_DEF_ attribute_name default_value;`
    pub(crate) fn parse(pair: Pair<Rule>) -> DbcResult<Self> {
        let mut pairs = pair.into_inner();
        let name = parse_str(next_rule(&mut pairs, Rule::attribute_name)?);
        let value = pairs.next().ok_or(DbcError::ParseError)?.try_into()?;
        expect_empty(&mut pairs)?;

        Ok(Self { name, value })
    }
}
