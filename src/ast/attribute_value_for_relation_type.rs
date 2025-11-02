use can_dbc_pest::{Pair, Rule};

use crate::parser::{expect_empty, next, next_rule, next_string, DbcError};
use crate::{AttributeValue, MessageId};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AttributeValueForRelationType {
    NodeToSignal {
        node_name: String,
        message_id: MessageId,
        signal_name: String,
        value: AttributeValue,
    },
    NodeToMessage {
        node_name: String,
        message_id: MessageId,
        value: AttributeValue,
    },
}

impl TryFrom<Pair<'_, Rule>> for AttributeValueForRelationType {
    type Error = DbcError;

    /// Parse `rel_object_data`: `ident ~ signal_var` | `ident ~ env_var_val` | `ident ~ message_id ~ num_str_value`
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        if value.as_rule() != Rule::rel_object_data {
            return Err(DbcError::ExpectedRule(
                Rule::rel_object_data,
                value.as_rule(),
            ));
        }

        let mut pairs = value.into_inner();
        let node_name = next_string(&mut pairs, Rule::ident)?;

        // Peek at the next rule to determine the structure
        let next_pair = next(&mut pairs)?;

        match next_pair.as_rule() {
            Rule::signal_var => {
                // BU_SG_REL_: ident ~ signal_var
                let mut signal_pairs = next_pair.into_inner();
                let message_id = next_rule(&mut signal_pairs, Rule::message_id)?.try_into()?;
                let signal_name = next_string(&mut signal_pairs, Rule::ident)?;
                let value = next(&mut signal_pairs)?.try_into()?;
                expect_empty(&signal_pairs)?;

                Ok(Self::NodeToSignal {
                    node_name,
                    message_id,
                    signal_name,
                    value,
                })
            }
            Rule::env_var_val => {
                // BU_EV_REL_: ident ~ env_var_val
                // TODO: This case is not yet defined in AttributeValueForRelationType
                // For now, return an error or add a new variant
                Err(DbcError::NotImplemented("BU_EV_REL_ not yet implemented"))
            }
            Rule::message_id => {
                // BU_BO_REL_: ident ~ message_id ~ num_str_value
                let message_id = next_pair.try_into()?;
                let value = next(&mut pairs)?.try_into()?;
                expect_empty(&pairs)?;

                Ok(Self::NodeToMessage {
                    node_name,
                    message_id,
                    value,
                })
            }
            other => Err(DbcError::UnknownRule(other)),
        }
    }
}
