use can_dbc_pest::{Pair, Pairs, Rule};

use crate::ast::MessageId;
use crate::parser::{
    inner_str, next, next_optional_rule, next_rule, next_string, parse_next_inner_str,
    single_inner, validated_inner, DbcError,
};

/// Object comments
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Comment {
    Node {
        name: String,
        comment: String,
    },
    Message {
        id: MessageId,
        comment: String,
    },
    Signal {
        message_id: MessageId,
        name: String,
        comment: String,
    },
    EnvVar {
        name: String,
        comment: String,
    },
    Plain {
        comment: String,
    },
}

impl TryFrom<Pair<'_, Rule>> for Comment {
    type Error = DbcError;

    /// Parse comment: `CM_ [BU_|BO_|SG_|EV_] object_name "comment_text";`
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut inner_pairs = validated_inner(value, Rule::comment)?;

        let pair = next(&mut inner_pairs)?;

        if pair.as_rule() == Rule::comment_plain {
            // Parse plain comment: `"comment"`
            let comment = inner_str(single_inner(pair, Rule::quoted_str)?);
            Ok(Comment::Plain { comment })
        } else {
            let rule = pair.as_rule();
            let mut inner = pair.into_inner();
            match rule {
                Rule::comment_signal => parse_signal_comment(inner),
                Rule::comment_message | Rule::comment_message_implicit => {
                    // Parse message comment: `BO_ <message_id> "comment"`
                    //      implicit comment: `<message_id> "comment"`
                    Ok(Comment::Message {
                        id: next_rule(&mut inner, Rule::message_id)?.try_into()?,
                        comment: parse_next_inner_str(&mut inner, Rule::quoted_str)?,
                    })
                }
                Rule::comment_node => {
                    // Parse node comment: `BU_ <node_name> "comment"`
                    Ok(Comment::Node {
                        name: next_string(&mut inner, Rule::node_name)?,
                        comment: parse_next_inner_str(&mut inner, Rule::quoted_str)?,
                    })
                }
                Rule::comment_env_var => {
                    // Parse environment variable comment: `EV_ <env_var_name> "comment"`
                    Ok(Comment::EnvVar {
                        name: next_string(&mut inner, Rule::env_var_name)?,
                        comment: parse_next_inner_str(&mut inner, Rule::quoted_str)?,
                    })
                }
                rule => Err(DbcError::UnknownRule(rule)),
            }
        }
    }
}

/// Parse signal comment: `SG_ <message_id> [<signal_name>] "comment"`
/// If `signal_name` is omitted, this is treated as a message comment.
fn parse_signal_comment(mut pairs: Pairs<Rule>) -> Result<Comment, DbcError> {
    let message_id = next_rule(&mut pairs, Rule::message_id)?.try_into()?;
    if let Some(name) = next_optional_rule(&mut pairs, Rule::signal_name) {
        // This is a proper signal comment with signal name
        Ok(Comment::Signal {
            message_id,
            name: name.as_str().to_string(),
            comment: parse_next_inner_str(&mut pairs, Rule::quoted_str)?,
        })
    } else {
        // No signal name - treat as message comment
        Ok(Comment::Message {
            id: message_id,
            comment: parse_next_inner_str(&mut pairs, Rule::quoted_str)?,
        })
    }
}
