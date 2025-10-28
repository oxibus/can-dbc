use can_dbc_pest::{Pair, Pairs, Rule};

use crate::ast::MessageId;
use crate::parser::{inner_str, next, next_rule, validated_inner, DbcError};

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

        match pair.as_rule() {
            Rule::comment_signal => parse_signal_comment(pair),
            Rule::comment_message | Rule::comment_message_implicit => parse_message_comment(pair),
            Rule::comment_node => parse_node_comment(pair),
            Rule::comment_env_var => parse_env_var_comment(pair),
            Rule::comment_plain => parse_plain_comment(pair),
            rule => Err(DbcError::UnknownRule(rule)),
        }
    }
}

/// Parse signal comment: `SG_ <message_id> [<signal_name>] "comment"`
/// If signal_name is omitted, this is treated as a message comment.
fn parse_signal_comment(pair: Pair<'_, Rule>) -> Result<Comment, DbcError> {
    let mut pairs = pair.into_inner();
    let message_id = next_rule(&mut pairs, Rule::message_id)?.try_into()?;
    let next_pair = next(&mut pairs)?;
    match next_pair.as_rule() {
        Rule::signal_name => {
            // This is a proper signal comment with signal name
            Ok(Comment::Signal {
                message_id,
                name: next_pair.as_str().to_string(),
                comment: inner_str(next_rule(&mut pairs, Rule::quoted_str)?),
            })
        }
        Rule::quoted_str => {
            // No signal name - treat as message comment
            Ok(Comment::Message {
                id: message_id,
                comment: inner_str(next_rule(&mut pairs, Rule::quoted_str)?),
            })
        }
        rule => Err(DbcError::UnknownRule(rule)),
    }
}

/// Parse message comment: `BO_ <message_id> "comment"`
fn parse_message_comment(pair: Pair<'_, Rule>) -> Result<Comment, DbcError> {
    let mut inner = pair.into_inner();
    let message_id = next_rule(&mut inner, Rule::message_id)?.try_into()?;
    let comment = inner_str(next_rule(&mut inner, Rule::quoted_str)?);

    Ok(Comment::Message {
        id: message_id,
        comment,
    })
}

/// Parse node comment: `BU_ <node_name> "comment"`
fn parse_node_comment(pair: Pair<'_, Rule>) -> Result<Comment, DbcError> {
    let mut inner = pair.into_inner();
    let node_name = next_ident(&mut inner)?;
    let comment = inner_str(next_rule(&mut inner, Rule::quoted_str)?);

    Ok(Comment::Node {
        name: node_name,
        comment,
    })
}

/// Parse environment variable comment: `EV_ <env_var_name> "comment"`
fn parse_env_var_comment(pair: Pair<'_, Rule>) -> Result<Comment, DbcError> {
    let mut inner = pair.into_inner();
    let env_var_name = next_ident(&mut inner)?;
    let comment = inner_str(next_rule(&mut inner, Rule::quoted_str)?);

    Ok(Comment::EnvVar {
        name: env_var_name,
        comment,
    })
}

/// Parse plain comment: `"comment"`
fn parse_plain_comment(pair: Pair<'_, Rule>) -> Result<Comment, DbcError> {
    // comment_plain contains a quoted_str, so we need to get the inner pair
    let mut inner = pair.into_inner();
    let comment = inner_str(next_rule(&mut inner, Rule::quoted_str)?);
    Ok(Comment::Plain { comment })
}

/// Helper to get next identifier string from pairs iterator
fn next_ident<'a>(iter: &'a mut Pairs<Rule>) -> Result<String, DbcError> {
    let pair = next(iter)?;
    match pair.as_rule() {
        Rule::signal_name | Rule::node_name | Rule::env_var_name | Rule::ident => {
            Ok(pair.as_str().to_string())
        }
        rule => Err(DbcError::UnknownRule(rule)),
    }
}
