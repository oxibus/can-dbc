use can_dbc_pest::{Pair, Rule};

use crate::parser::{expect_empty, inner_str, next_rule};
use crate::{DbcError, DbcResult, NumericValue};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AttributeValueType {
    Int(NumericValue, NumericValue),
    Hex(NumericValue, NumericValue),
    Float(NumericValue, NumericValue),
    String,
    Enum(Vec<String>),
}

impl TryFrom<Pair<'_, Rule>> for AttributeValueType {
    type Error = DbcError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let rule = pair.as_rule();
        Ok(match rule {
            Rule::attribute_type_int | Rule::attribute_type_hex | Rule::attribute_type_float => {
                let mut pairs = pair.into_inner();
                let min = next_rule(&mut pairs, Rule::minimum)?.as_str().parse()?;
                let max = next_rule(&mut pairs, Rule::maximum)?.as_str().parse()?;
                expect_empty(&pairs)?;
                match rule {
                    Rule::attribute_type_int => AttributeValueType::Int(min, max),
                    Rule::attribute_type_hex => AttributeValueType::Hex(min, max),
                    Rule::attribute_type_float => AttributeValueType::Float(min, max),
                    _ => unreachable!(),
                }
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
