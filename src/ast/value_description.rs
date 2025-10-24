use can_dbc_pest::{Pair, Rule};

use crate::ast::{MessageId, ValDescription};
use crate::parser;
use crate::parser::DbcResult;

/// Encoding for signal raw values.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ValueDescription {
    Signal {
        message_id: MessageId,
        name: String,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
        value_descriptions: Vec<ValDescription>,
    },
    EnvironmentVariable {
        name: String,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
        value_descriptions: Vec<ValDescription>,
    },
}

impl ValueDescription {
    /// Parse value description: `VAL_ message_id signal_name value1 "description1" value2 "description2" ... ;`
    pub(crate) fn parse(pair: Pair<Rule>) -> DbcResult<ValueDescription> {
        let mut pairs = pair.into_inner();

        // Check if first item is message_id (optional)
        let mut message_id = None;
        if let Some(first_pair) = pairs.next() {
            if first_pair.as_rule() == Rule::message_id {
                message_id = Some(parser::parse_uint(first_pair)? as u32);
            } else {
                // Put it back and treat as signal_name (environment variable case)
                let signal_name = first_pair.as_str().to_string();
                let mut descriptions = Vec::new();
                for pair2 in pairs {
                    if pair2.as_rule() == Rule::table_value_description {
                        descriptions.push(ValDescription::parse_table_value_description(pair2)?);
                    }
                }
                return Ok(ValueDescription::EnvironmentVariable {
                    name: signal_name,
                    value_descriptions: descriptions,
                });
            }
        }

        let signal_name = parser::next_rule(&mut pairs, Rule::signal_name)?
            .as_str()
            .to_string();

        // Collect table value descriptions
        let mut descriptions = Vec::new();
        for pair2 in pairs {
            if pair2.as_rule() == Rule::table_value_description {
                descriptions.push(ValDescription::parse_table_value_description(pair2)?);
            }
        }

        if let Some(msg_id) = message_id {
            let msg_id = if msg_id & (1 << 31) != 0 {
                MessageId::Extended(msg_id & 0x1FFF_FFFF)
            } else {
                MessageId::Standard(msg_id as u16)
            };
            Ok(ValueDescription::Signal {
                message_id: msg_id,
                name: signal_name,
                value_descriptions: descriptions,
            })
        } else {
            Ok(ValueDescription::EnvironmentVariable {
                name: signal_name,
                value_descriptions: descriptions,
            })
        }
    }
}
