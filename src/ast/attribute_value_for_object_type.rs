use can_dbc_pest::{Pairs, Rule};

use crate::ast::{AttributeValue, MessageId};
use crate::parser::{expect_empty, next, next_rule, next_string};
use crate::DbcError;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) enum AttributeValueForObjectType {
    Raw(AttributeValue),
    NetworkNode(String, AttributeValue),
    MessageDefinition(MessageId, Option<AttributeValue>),
    Signal(MessageId, String, AttributeValue),
    EnvVariable(String, AttributeValue),
}

impl TryFrom<Pairs<'_, Rule>> for AttributeValueForObjectType {
    type Error = DbcError;

    fn try_from(mut value: Pairs<Rule>) -> Result<Self, Self::Error> {
        // Expect exactly one remaining pair (the object-specific value)
        let pair = value.next().ok_or(DbcError::NoMoreRules)?;
        if let Some(more) = value.next() {
            return Err(DbcError::ExpectedEmpty(more.as_rule()));
        }

        if matches!(&pair.as_rule(), Rule::quoted_str | Rule::number) {
            return Ok(AttributeValueForObjectType::Raw(pair.try_into()?));
        }

        let rule = pair.as_rule();
        let mut pairs = pair.into_inner();

        let res = match rule {
            Rule::node_var_val => AttributeValueForObjectType::NetworkNode(
                next_string(&mut pairs, Rule::node_name)?,
                next(&mut pairs)?.try_into()?,
            ),
            Rule::msg_var_val => AttributeValueForObjectType::MessageDefinition(
                next_rule(&mut pairs, Rule::message_id)?.try_into()?,
                Some(next(&mut pairs)?.try_into()?),
            ),
            Rule::signal_var => AttributeValueForObjectType::Signal(
                next_rule(&mut pairs, Rule::message_id)?.try_into()?,
                next_string(&mut pairs, Rule::ident)?,
                next(&mut pairs)?.try_into()?,
            ),
            Rule::env_var_val => AttributeValueForObjectType::EnvVariable(
                next_string(&mut pairs, Rule::env_var_name)?,
                next(&mut pairs)?.try_into()?,
            ),
            v => return Err(DbcError::UnknownRule(v)),
        };

        expect_empty(&pairs)?;
        Ok(res)
    }
}
