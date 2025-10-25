use can_dbc_pest::{Pair, Rule};

use crate::ast::MessageId;
use crate::parser::{collect_strings, next_rule, next_string, parse_uint, DbcResult};

/// Signal groups define a group of signals within a message
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SignalGroups {
    pub message_id: MessageId,
    pub name: String,
    pub repetitions: u64,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub signal_names: Vec<String>,
}

impl SignalGroups {
    /// Parse signal group: `SIG_GROUP_ message_id group_name multiplexer_id : signal1 signal2 ... ;`
    pub(crate) fn parse(pair: Pair<Rule>) -> DbcResult<SignalGroups> {
        let mut pairs = pair.into_inner();

        let message_id = parse_uint(next_rule(&mut pairs, Rule::message_id)?)? as u32;
        let name = next_string(&mut pairs, Rule::group_name)?;
        let repetitions = parse_uint(next_rule(&mut pairs, Rule::multiplexer_id)?)?;
        let signal_names = collect_strings(&mut pairs, Rule::signal_name)?;

        Ok(SignalGroups {
            message_id: MessageId::parse(message_id),
            name,
            repetitions,
            signal_names,
        })
    }
}
