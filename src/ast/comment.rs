use can_dbc_pest::{Pair, Rule};

use crate::ast::MessageId;
use crate::parser::{
    inner_str, parse_uint, single_inner, single_inner_str, validated_inner, DbcError,
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
        let inner_pairs = validated_inner(value, Rule::comment)?;

        let mut comment = String::new();
        let mut message_id = None;
        let mut signal_name = None;
        let mut node_name = None;
        let mut env_var_name = None;

        for pair in inner_pairs {
            match pair.as_rule() {
                Rule::quoted_str => comment = inner_str(pair),
                Rule::msg_var => {
                    message_id = Some(parse_uint(single_inner(pair, Rule::message_id)?)? as u32);
                }
                Rule::msg_var_str => {
                    // msg_var_str contains a message id and a quoted string
                    for sub in pair.into_inner() {
                        match sub.as_rule() {
                            Rule::message_id => {
                                message_id = Some(parse_uint(sub)? as u32);
                            }
                            Rule::quoted_str => comment = inner_str(sub),
                            v => return Err(DbcError::UnknownRule(v)),
                        }
                    }
                }
                Rule::node_var_str => {
                    // node_var_str contains node name and quoted_str
                    for sub in pair.into_inner() {
                        match sub.as_rule() {
                            Rule::node_name => node_name = Some(sub.as_str().to_string()),
                            Rule::quoted_str => comment = inner_str(sub),
                            v => return Err(DbcError::UnknownRule(v)),
                        }
                    }
                }
                Rule::env_var => {
                    env_var_name = Some(single_inner_str(pair, Rule::env_var_name)?);
                }
                Rule::env_var_str => {
                    for sub in pair.into_inner() {
                        match sub.as_rule() {
                            Rule::env_var_name => env_var_name = Some(sub.as_str().to_string()),
                            Rule::quoted_str => comment = inner_str(sub),
                            v => return Err(DbcError::UnknownRule(v)),
                        }
                    }
                }
                Rule::signal_var => {
                    for sub_pair in pair.into_inner() {
                        match sub_pair.as_rule() {
                            Rule::message_id => message_id = Some(parse_uint(sub_pair)? as u32),
                            Rule::ident => signal_name = Some(sub_pair.as_str().to_string()),
                            v => return Err(DbcError::UnknownRule(v)),
                        }
                    }
                }
                Rule::message_id => message_id = Some(parse_uint(pair)? as u32),
                Rule::signal_name => signal_name = Some(pair.as_str().to_string()),
                Rule::node_name => node_name = Some(pair.as_str().to_string()),
                Rule::env_var_name => env_var_name = Some(pair.as_str().to_string()),
                _ => return Err(DbcError::UnknownRule(pair.as_rule())),
            }
        }

        // Determine comment type based on parsed components
        // The grammar structure tells us:
        // - If we have message_id + signal_name => Signal comment
        // - If we have message_id only => Message comment
        // - If we have node_name => Node comment
        // - If we have env_var_name => Environment variable comment
        // - Otherwise => Plain comment

        let message_id = message_id.map(|id| {
            if id & (1 << 31) != 0 {
                MessageId::Extended(id & 0x1FFF_FFFF)
            } else {
                MessageId::Standard(id as u16)
            }
        });

        Ok(match (message_id, signal_name, node_name, env_var_name) {
            (Some(message_id), Some(name), _, _) => Self::Signal {
                message_id,
                name,
                comment,
            },
            (Some(id), None, _, _) => Self::Message { id, comment },
            (_, _, Some(name), _) => Self::Node { name, comment },
            (_, _, _, Some(name)) => Self::EnvVar { name, comment },
            _ if !comment.is_empty() => Self::Plain { comment },
            _ => return Err(DbcError::ParseError),
        })
    }
}
