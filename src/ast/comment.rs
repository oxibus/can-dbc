use can_dbc_pest::{Pair, Rule};

use crate::ast::MessageId;
use crate::parser::{parse_str, parse_uint, DbcResult};

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
        let mut comment_text = String::new();
        let mut message_id = None;
        let mut signal_name = None;
        let mut node_name = None;
        let mut env_var_name = None;

        // Process all inner pairs to extract information
        for pairs in pair.into_inner() {
            match pairs.as_rule() {
                Rule::quoted_str => comment_text = parse_str(pairs),
                Rule::msg_var => {
                    // msg_var contains msg_literal ~ message_id
                    for sub_pair in pairs.into_inner() {
                        if sub_pair.as_rule() == Rule::message_id {
                            message_id = Some(parse_uint(sub_pair)? as u32);
                        }
                    }
                }
                Rule::node_var => {
                    // node_var contains node_literal ~ node_name
                    for sub_pair in pairs.into_inner() {
                        if sub_pair.as_rule() == Rule::node_name {
                            node_name = Some(sub_pair.as_str().to_string());
                        }
                    }
                }
                Rule::env_var => {
                    // env_var contains env_literal ~ env_var_name
                    for sub_pair in pairs.into_inner() {
                        if sub_pair.as_rule() == Rule::env_var_name {
                            env_var_name = Some(sub_pair.as_str().to_string());
                        }
                    }
                }
                Rule::signal_var => {
                    // signal_var contains signal_literal ~ message_id ~ ident
                    for sub_pair in pairs.into_inner() {
                        match sub_pair.as_rule() {
                            Rule::message_id => {
                                message_id = Some(parse_uint(sub_pair)? as u32);
                            }
                            Rule::ident => {
                                signal_name = Some(sub_pair.as_str().to_string());
                            }
                            _ => {}
                        }
                    }
                }
                Rule::message_id => message_id = Some(parse_uint(pairs)? as u32),
                Rule::signal_name => signal_name = Some(pairs.as_str().to_string()),
                Rule::node_name => node_name = Some(pairs.as_str().to_string()),
                Rule::env_var_name => env_var_name = Some(pairs.as_str().to_string()),
                other => panic!("What is this? {other:?}"),
            }
        }

        // Determine comment type based on parsed components
        // The grammar structure tells us:
        // - If we have message_id + signal_name => Signal comment
        // - If we have message_id only => Message comment
        // - If we have node_name => Node comment
        // - If we have env_var_name => Environment variable comment
        // - Otherwise => Plain comment

        if let Some(msg_id) = message_id {
            let msg_id = if msg_id & (1 << 31) != 0 {
                MessageId::Extended(msg_id & 0x1FFF_FFFF)
            } else {
                MessageId::Standard(msg_id as u16)
            };

            return if let Some(sig_name) = signal_name {
                // Signal comment: CM_ SG_ message_id signal_name "comment"
                Ok(Some(Comment::Signal {
                    message_id: msg_id,
                    name: sig_name,
                    comment: comment_text,
                }))
            } else {
                // Message comment: CM_ BO_ message_id "comment"
                Ok(Some(Comment::Message {
                    id: msg_id,
                    comment: comment_text,
                }))
            };
        } else if let Some(node) = node_name {
            // Node comment: CM_ BU_ node_name "comment"
            return Ok(Some(Comment::Node {
                name: node,
                comment: comment_text,
            }));
        } else if let Some(env_var) = env_var_name {
            // Environment variable comment: CM_ EV_ env_var_name "comment"
            return Ok(Some(Comment::EnvVar {
                name: env_var,
                comment: comment_text,
            }));
        } else if !comment_text.is_empty() {
            // Plain comment: CM_ "comment"
            return Ok(Some(Comment::Plain {
                comment: comment_text,
            }));
        }

        Ok(None)
    }
}
