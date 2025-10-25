use can_dbc_pest::{Pair, Rule};

use crate::ast::AttributeValuedForObjectType;
use crate::parser::{
    expect_empty, next_rule, next_string, parse_float, parse_str, single_rule, DbcResult,
};
use crate::{AttributeValue, MessageId};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AttributeValueForObject {
    pub name: String,
    pub value: AttributeValuedForObjectType,
}

impl AttributeValueForObject {
    /// Parse attribute value: `BA_ attribute_name [object_type] object_name value;`
    pub(crate) fn parse(pair: Pair<Rule>) -> DbcResult<Self> {
        let mut name = String::new();
        let mut object_type = None;
        let mut message_id: Option<MessageId> = None;
        let mut signal_name = None;
        let mut node_name = None;
        let mut env_var_name = None;
        let mut value = None;

        for pairs in pair.into_inner() {
            match pairs.as_rule() {
                Rule::attribute_name => name = parse_str(pairs),
                // num_str_value is a silent rule, so we get quoted_str or number directly
                Rule::quoted_str => value = Some(AttributeValue::String(parse_str(pairs))),
                Rule::number => value = Some(AttributeValue::Double(parse_float(pairs)?)),
                Rule::node_var => {
                    object_type = Some(pairs.as_rule());
                    // Parse the node name from the inner pairs
                    // node_var contains: node_literal ~ node_name
                    // node_literal is silent (_), so we get node_name directly
                    node_name = Some(single_rule(pairs, Rule::node_name)?.as_str().to_string());
                }
                Rule::msg_var => {
                    object_type = Some(pairs.as_rule());
                    // Parse the message ID from the inner pairs
                    message_id = Some(single_rule(pairs, Rule::message_id)?.try_into()?);
                }
                Rule::signal_var => {
                    object_type = Some(pairs.as_rule());
                    // Parse the message ID and signal name from the inner pairs
                    let mut inner_pairs = pairs.into_inner();
                    message_id = Some(next_rule(&mut inner_pairs, Rule::message_id)?.try_into()?);
                    signal_name = Some(next_string(&mut inner_pairs, Rule::ident)?);
                    expect_empty(&mut inner_pairs)?;
                }
                Rule::env_var => {
                    object_type = Some(pairs.as_rule());
                    // Parse the environment variable name from the inner pairs
                    // env_var contains: env_literal ~ env_var_name
                    // env_literal is silent (_), so we get env_var_name directly
                    let v = single_rule(pairs, Rule::env_var_name)?;
                    env_var_name = Some(v.as_str().to_string());
                }
                other => panic!("What is this? {other:?}"),
            }
        }

        let value = value.unwrap_or(AttributeValue::String(String::new()));

        // Determine attribute value type based on parsed components
        let value = match object_type {
            Some(Rule::signal_var) => {
                if let (Some(msg_id), Some(sig_name)) = (message_id, signal_name) {
                    AttributeValuedForObjectType::Signal(msg_id, sig_name, value)
                } else {
                    todo!()
                    // AttributeValuedForObjectType::Raw(value)
                }
            }
            Some(Rule::msg_var) => {
                if let Some(msg_id) = message_id {
                    AttributeValuedForObjectType::MessageDefinition(msg_id, Some(value))
                } else {
                    AttributeValuedForObjectType::Raw(value)
                }
            }
            Some(Rule::node_var) => {
                if let Some(node) = node_name {
                    AttributeValuedForObjectType::NetworkNode(node, value)
                } else {
                    AttributeValuedForObjectType::Raw(value)
                }
            }
            Some(Rule::env_var) => {
                if let Some(env_var) = env_var_name {
                    AttributeValuedForObjectType::EnvVariable(env_var, value)
                } else {
                    AttributeValuedForObjectType::Raw(value)
                }
            }
            _ => AttributeValuedForObjectType::Raw(value),
        };

        Ok(Self { name, value })
    }
}
