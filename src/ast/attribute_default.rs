use can_dbc_pest::{Pair, Rule};

use crate::ast::AttributeValue;
use crate::{parser, DbcError, DbcResult};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttributeDefault {
    pub name: String,
    pub value: AttributeValue,
}

impl AttributeDefault {
    /// Parse attribute default: `BA_DEF_DEF_ attribute_name default_value;`
    pub(crate) fn parse(pair: Pair<Rule>) -> DbcResult<AttributeDefault> {
        let mut inner_pairs = pair.into_inner();

        let attribute_name =
            parser::parse_str(parser::next_rule(&mut inner_pairs, Rule::attribute_name)?);

        // Parse the value - could be quoted_str or number (num_str_value is silent)
        let value_pair = inner_pairs.next().ok_or(DbcError::ParseError)?;
        let default_value = match value_pair.as_rule() {
            Rule::quoted_str => AttributeValue::String(parser::parse_str(value_pair)),
            Rule::number => AttributeValue::Double(parser::parse_float(value_pair)?),
            _ => return Err(DbcError::ParseError),
        };

        // Don't use expect_empty here as there might be comments or whitespace

        Ok(AttributeDefault {
            name: attribute_name,
            value: default_value,
        })
    }
}
