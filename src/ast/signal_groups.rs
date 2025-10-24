use can_dbc_pest::{Pair, Rule};

use crate::ast::MessageId;
use crate::parser;
use crate::parser::DbcResult;

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
        let mut inner_pairs = pair.into_inner();

        let message_id =
            parser::parse_uint(parser::next_rule(&mut inner_pairs, Rule::message_id)?)? as u32;
        let group_name = parser::next_rule(&mut inner_pairs, Rule::group_name)?
            .as_str()
            .to_string();
        let repetitions =
            parser::parse_uint(parser::next_rule(&mut inner_pairs, Rule::multiplexer_id)?)?;

        // Collect remaining signal names
        let mut signal_names = Vec::new();
        for pair2 in inner_pairs {
            if pair2.as_rule() == Rule::signal_name {
                signal_names.push(pair2.as_str().to_string());
            }
        }

        let msg_id = if message_id & (1 << 31) != 0 {
            MessageId::Extended(message_id & 0x1FFF_FFFF)
        } else {
            MessageId::Standard(message_id as u16)
        };

        Ok(SignalGroups {
            message_id: msg_id,
            name: group_name,
            repetitions,
            signal_names,
        })
    }
}
