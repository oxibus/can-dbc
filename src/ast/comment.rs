use can_dbc_pest::{Pair, Rule};

use crate::ast::MessageId;
use crate::parser::{parse_str, parse_uint, single_rule, DbcResult};

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

impl Comment {
    /// Parse comment: `CM_ [BU_|BO_|SG_|EV_] object_name "comment_text";`
    pub(crate) fn parse(pair: Pair<Rule>) -> DbcResult<Option<Comment>> {
        let mut comment = String::new();
        let mut message_id = None;
        let mut signal_name = None;
        let mut node_name = None;
        let mut env_var_name = None;

        for pairs in pair.into_inner() {
            match pairs.as_rule() {
                Rule::quoted_str => comment = parse_str(pairs),
                Rule::msg_var => {
                    message_id = Some(parse_uint(single_rule(pairs, Rule::message_id)?)? as u32)
                }
                Rule::node_var => {
                    node_name = Some(single_rule(pairs, Rule::node_name)?.as_str().to_string())
                }
                Rule::env_var => {
                    env_var_name =
                        Some(single_rule(pairs, Rule::env_var_name)?.as_str().to_string())
                }
                Rule::signal_var => {
                    for sub_pair in pairs.into_inner() {
                        match sub_pair.as_rule() {
                            Rule::message_id => message_id = Some(parse_uint(sub_pair)? as u32),
                            Rule::ident => signal_name = Some(sub_pair.as_str().to_string()),
                            _ => {}
                        }
                    }
                }
                Rule::message_id => message_id = Some(parse_uint(pairs)? as u32),
                Rule::signal_name => signal_name = Some(pairs.as_str().to_string()),
                Rule::node_name => node_name = Some(pairs.as_str().to_string()),
                Rule::env_var_name => env_var_name = Some(pairs.as_str().to_string()),
                _ => panic!("Unexpected rule: {:?}", pairs.as_rule()),
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
            (Some(message_id), Some(name), _, _) => Some(Comment::Signal {
                message_id,
                name,
                comment,
            }),
            (Some(id), None, _, _) => Some(Comment::Message { id, comment }),
            (_, _, Some(name), _) => Some(Comment::Node { name, comment }),
            (_, _, _, Some(name)) => Some(Comment::EnvVar { name, comment }),
            _ if !comment.is_empty() => Some(Comment::Plain { comment }),
            _ => None,
        })
    }
}
