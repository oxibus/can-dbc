use can_dbc_pest::{Pair, Rule};

use crate::ast::MessageId;
use crate::parser::{next_rule, parse_uint, DbcResult};

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
        let name = next_rule(&mut pairs, Rule::group_name)?
            .as_str()
            .to_string();
        let repetitions = parse_uint(next_rule(&mut pairs, Rule::multiplexer_id)?)?;
        let signal_names = pairs
            .filter(|pair| pair.as_rule() == Rule::signal_name)
            .map(|pair| pair.as_str().to_string())
            .collect();

        let message_id = if message_id & (1 << 31) != 0 {
            MessageId::Extended(message_id & 0x1FFF_FFFF)
        } else {
            MessageId::Standard(message_id as u16)
        };

        Ok(SignalGroups {
            message_id,
            name,
            repetitions,
            signal_names,
        })
    }
}
