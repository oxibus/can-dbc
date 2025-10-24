use can_dbc_pest::{Pair, Rule};

use crate::ast::AttributeValuedForObjectType;
use crate::{parser, AttributeValue, DbcResult, MessageId};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttributeValueForObject {
    pub name: String,
    pub value: AttributeValuedForObjectType,
}

/// Parse attribute value: `BA_ attribute_name [object_type] object_name value;`
pub(crate) fn parse_attribute_value(pair: Pair<Rule>) -> DbcResult<AttributeValueForObject> {
    let mut attribute_name = String::new();
    let mut object_type = None;
    let mut message_id = None;
    let mut signal_name = None;
    let mut node_name = None;
    let mut env_var_name = None;
    let mut value = None;

    for pair2 in pair.into_inner() {
        match pair2.as_rule() {
            Rule::attribute_name => {
                attribute_name = parser::parse_str(pair2);
            }
            // num_str_value is a silent rule, so we get quoted_str or number directly
            Rule::quoted_str => {
                value = Some(AttributeValue::String(parser::parse_str(pair2)));
            }
            Rule::number => {
                value = Some(AttributeValue::Double(parser::parse_float(pair2)?));
            }
            Rule::node_var => {
                object_type = Some("node");
                // Parse the node name from the inner pairs
                // node_var contains: node_literal ~ node_name
                // node_literal is silent (_), so we get node_name directly
                for sub_pair in pair2.into_inner() {
                    if sub_pair.as_rule() == Rule::node_name {
                        node_name = Some(sub_pair.as_str().to_string());
                    }
                }
            }
            Rule::msg_var => {
                object_type = Some("message");
                // Parse the message ID from the inner pairs
                for sub_pair in pair2.into_inner() {
                    if sub_pair.as_rule() == Rule::message_id {
                        message_id = Some(parser::parse_uint(sub_pair)? as u32);
                    }
                }
            }
            Rule::signal_var => {
                object_type = Some("signal");
                // Parse the message ID and signal name from the inner pairs
                for sub_pair in pair2.into_inner() {
                    match sub_pair.as_rule() {
                        Rule::message_id => {
                            message_id = Some(parser::parse_uint(sub_pair)? as u32);
                        }
                        Rule::ident => {
                            signal_name = Some(sub_pair.as_str().to_string());
                        }
                        other => panic!("What is this? {other:?}"),
                    }
                }
            }
            Rule::env_var => {
                object_type = Some("env_var");
                // Parse the environment variable name from the inner pairs
                // env_var contains: env_literal ~ env_var_name
                // env_literal is silent (_), so we get env_var_name directly
                for sub_pair in pair2.into_inner() {
                    if sub_pair.as_rule() == Rule::env_var_name {
                        env_var_name = Some(sub_pair.as_str().to_string());
                    }
                }
            }
            other => panic!("What is this? {other:?}"),
        }
    }

    let value = value.unwrap_or(AttributeValue::String(String::new()));

    // Determine attribute value type based on parsed components
    match object_type {
        Some("signal") => {
            if let (Some(msg_id), Some(sig_name)) = (message_id, signal_name) {
                let msg_id = if msg_id & (1 << 31) != 0 {
                    MessageId::Extended(msg_id & 0x1FFF_FFFF)
                } else {
                    MessageId::Standard(msg_id as u16)
                };
                Ok(AttributeValueForObject {
                    name: attribute_name,
                    value: AttributeValuedForObjectType::Signal(msg_id, sig_name, value),
                })
            } else {
                todo!()
                // Ok(AttributeValueForObject {
                //     name: attribute_name,
                //     value: AttributeValuedForObjectType::Raw(value),
                // })
            }
        }
        Some("message") => {
            if let Some(msg_id) = message_id {
                let msg_id = if msg_id & (1 << 31) != 0 {
                    MessageId::Extended(msg_id & 0x1FFF_FFFF)
                } else {
                    MessageId::Standard(msg_id as u16)
                };
                Ok(AttributeValueForObject {
                    name: attribute_name,
                    value: AttributeValuedForObjectType::MessageDefinition(msg_id, Some(value)),
                })
            } else {
                Ok(AttributeValueForObject {
                    name: attribute_name,
                    value: AttributeValuedForObjectType::Raw(value),
                })
            }
        }
        Some("node") => {
            if let Some(node) = node_name {
                Ok(AttributeValueForObject {
                    name: attribute_name,
                    value: AttributeValuedForObjectType::NetworkNode(node, value),
                })
            } else {
                Ok(AttributeValueForObject {
                    name: attribute_name,
                    value: AttributeValuedForObjectType::Raw(value),
                })
            }
        }
        Some("env_var") => {
            if let Some(env_var) = env_var_name {
                Ok(AttributeValueForObject {
                    name: attribute_name,
                    value: AttributeValuedForObjectType::EnvVariable(env_var, value),
                })
            } else {
                Ok(AttributeValueForObject {
                    name: attribute_name,
                    value: AttributeValuedForObjectType::Raw(value),
                })
            }
        }
        _ => Ok(AttributeValueForObject {
            name: attribute_name,
            value: AttributeValuedForObjectType::Raw(value),
        }),
    }
}
