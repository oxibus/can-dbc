use can_dbc_pest::{Pair, Rule};

use crate::ast::AttributeValue;
use crate::parser::{expect_empty, next_rule, parse_float, parse_str, DbcError, DbcResult};

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

        // Parse the value - could be quoted_str or number (num_str_value is silent)
        let value_pair = pairs.next().ok_or(DbcError::ParseError)?;
        let value = match value_pair.as_rule() {
            Rule::quoted_str => AttributeValue::String(parse_str(value_pair)),
            Rule::number => AttributeValue::Double(parse_float(value_pair)?),
            _ => return Err(DbcError::ParseError),
        };
        expect_empty(&mut pairs)?;

        Ok(Self { name, value })
    }
}
