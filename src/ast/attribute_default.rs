use can_dbc_pest::{Pair, Rule};

use crate::ast::AttributeValue;
use crate::parser::{expect_empty, inner_str, next, next_rule, validated_inner, DbcError};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttributeDefault {
    pub name: String,
    pub value: AttributeValue,
}

impl TryFrom<Pair<'_, Rule>> for AttributeDefault {
    type Error = DbcError;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut pairs = validated_inner(value, Rule::ba_def_def)?;
        let name = inner_str(next_rule(&mut pairs, Rule::attribute_name)?);
        let value = next(&mut pairs)?.try_into()?;
        expect_empty(&pairs)?;

        Ok(Self { name, value })
    }
}
