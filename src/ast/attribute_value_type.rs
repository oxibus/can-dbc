use can_dbc_pest::{Pair, Rule};

use crate::parser::{inner_str, next_rule, parse_float, parse_int};
use crate::{DbcError, DbcResult};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AttributeValueType {
    Int(i64, i64),
    Hex(i64, i64),
    Float(f64, f64),
    String,
    Enum(Vec<String>),
}

impl TryFrom<Pair<'_, Rule>> for AttributeValueType {
    type Error = DbcError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let rule = pair.as_rule();
        Ok(match rule {
            Rule::attribute_type_int | Rule::attribute_type_hex => {
                let mut pairs = pair.into_inner();
                let min = parse_int(&next_rule(&mut pairs, Rule::minimum)?)?;
                let max = parse_int(&next_rule(&mut pairs, Rule::maximum)?)?;
                if rule == Rule::attribute_type_int {
                    AttributeValueType::Int(min, max)
                } else {
                    AttributeValueType::Hex(min, max)
                }
            }
            Rule::attribute_type_float => {
                let mut pairs = pair.into_inner();
                let min = parse_float(&next_rule(&mut pairs, Rule::minimum)?)?;
                let max = parse_float(&next_rule(&mut pairs, Rule::maximum)?)?;
                AttributeValueType::Float(min, max)
            }
            Rule::attribute_type_string => AttributeValueType::String,
            Rule::attribute_type_enum => {
                let enum_values: DbcResult<_> = pair
                    .into_inner()
                    .map(|pair| {
                        if pair.as_rule() == Rule::quoted_str {
                            Ok(inner_str(pair))
                        } else {
                            Err(DbcError::ExpectedRule(Rule::quoted_str, pair.as_rule()))
                        }
                    })
                    .collect();
                AttributeValueType::Enum(enum_values?)
            }
            v => return Err(DbcError::UnknownRule(v)),
        })
    }
}
