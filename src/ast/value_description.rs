use can_dbc_pest::{Pair, Rule};

use crate::ast::{MessageId, ValDescription};
use crate::parser::{collect_all, collect_expected, next_string, parse_uint, DbcResult};

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
                message_id = Some(parse_uint(first_pair)? as u32);
            } else {
                // Put it back and treat as signal_name (environment variable case)
                let name = first_pair.as_str().to_string();
                let value_descriptions =
                    collect_expected(&mut pairs, Rule::table_value_description)?;
                return Ok(ValueDescription::EnvironmentVariable {
                    name,
                    value_descriptions,
                });
            }
        }

        let name = next_string(&mut pairs, Rule::signal_name)?;
        let value_descriptions = collect_expected(&mut pairs, Rule::table_value_description)?;

        if let Some(msg_id) = message_id {
            Ok(ValueDescription::Signal {
                message_id: MessageId::parse(msg_id),
                name,
                value_descriptions,
            })
        } else {
            Ok(ValueDescription::EnvironmentVariable {
                name,
                value_descriptions,
            })
        }
    }
}
