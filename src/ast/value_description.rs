use can_dbc_pest::{Pair, Rule};

use crate::ast::{MessageId, ValDescription};
use crate::parser::{collect_expected, next_string, validated_inner, DbcError};

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

impl TryFrom<Pair<'_, Rule>> for ValueDescription {
    type Error = DbcError;

    /// Parse value description: `VAL_ message_id signal_name value1 "description1" value2 "description2" ... ;`
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut pairs = validated_inner(value, Rule::value_table_def)?;

        // Check if first item is message_id (optional)
        let mut message_id = None;
        if let Some(first_pair) = pairs.next() {
            if first_pair.as_rule() == Rule::message_id {
                message_id = Some(first_pair.try_into()?);
            } else {
                // Put it back and treat as signal_name (environment variable case)
                let name = first_pair.as_str().to_string();
                let value_descriptions =
                    collect_expected(&mut pairs, Rule::table_value_description)?;
                return Ok(Self::EnvironmentVariable {
                    name,
                    value_descriptions,
                });
            }
        }

        let name = next_string(&mut pairs, Rule::signal_name)?;
        let value_descriptions = collect_expected(&mut pairs, Rule::table_value_description)?;

        if let Some(message_id) = message_id {
            Ok(Self::Signal {
                message_id,
                name,
                value_descriptions,
            })
        } else {
            Ok(Self::EnvironmentVariable {
                name,
                value_descriptions,
            })
        }
    }
}
