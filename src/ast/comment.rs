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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

    #[test]
    fn signal_comment_test() {
        let def = r#"
CM_ SG_ 193 KLU_R_X "This is a signal comment test";
"#;
        let exp = Comment::Signal {
            message_id: MessageId::Standard(193),
            name: "KLU_R_X".to_string(),
            comment: "This is a signal comment test".to_string(),
        };
        let val = test_into::<Comment>(def.trim_start(), Rule::comment);
        assert_eq!(val, exp);
    }

    #[test]
    fn message_definition_comment_test() {
        let def = r#"
CM_ BO_ 34544 "Some Message comment";
"#;
        let exp = Comment::Message {
            id: MessageId::Standard(34544),
            comment: "Some Message comment".to_string(),
        };
        let val = test_into::<Comment>(def.trim_start(), Rule::comment);
        assert_eq!(val, exp);
    }

    #[test]
    fn node_comment_test() {
        let def = r#"
CM_ BU_ network_node "Some network node comment";
"#;
        let exp = Comment::Node {
            name: "network_node".to_string(),
            comment: "Some network node comment".to_string(),
        };
        let val = test_into::<Comment>(def.trim_start(), Rule::comment);
        assert_eq!(val, exp);
    }

    #[test]
    fn env_var_comment_test() {
        let def = r#"
CM_ EV_ ENVXYZ "Some env var name comment";
"#;
        let exp = Comment::EnvVar {
            name: "ENVXYZ".to_string(),
            comment: "Some env var name comment".to_string(),
        };
        let val = test_into::<Comment>(def.trim_start(), Rule::comment);
        assert_eq!(val, exp);
    }

    #[test]
    fn signal_comment_with_escaped_characters_test() {
        let def = r#"
CM_ SG_ 2147548912 FooBar "Foo\\ \n \"Bar\"";
"#;
        let exp = Comment::Signal {
            message_id: MessageId::Extended(65264),
            name: "FooBar".to_string(),
            comment: r#"Foo\\ \n \"Bar\""#.to_string(),
        };
        let val = test_into::<Comment>(def.trim_start(), Rule::comment);
        assert_eq!(val, exp);
    }

    #[test]
    fn empty_signal_comment_test() {
        let def = r#"
CM_ SG_ 2147548912 FooBar "";
"#;
        let exp = Comment::Signal {
            message_id: MessageId::Extended(65264),
            name: "FooBar".to_string(),
            comment: String::new(),
        };
        let val = test_into::<Comment>(def.trim_start(), Rule::comment);
        assert_eq!(val, exp);
    }
}
