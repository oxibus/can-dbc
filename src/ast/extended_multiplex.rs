use can_dbc_pest::{Pair, Rule};

use crate::ast::{ExtendedMultiplexMapping, MessageId};
use crate::parser;
use crate::parser::DbcResult;

/// Mapping between multiplexors and multiplexed signals
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtendedMultiplex {
    pub message_id: MessageId,
    pub signal_name: String,
    pub multiplexor_signal_name: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub mappings: Vec<ExtendedMultiplexMapping>,
}

impl ExtendedMultiplex {
    /// Parse extended multiplex: `SG_MUL_VAL_ message_id signal_name multiplexor_name value_pairs;`
    pub(crate) fn parse(pair: Pair<Rule>) -> DbcResult<ExtendedMultiplex> {
        let mut pairs = pair.into_inner();

        let message_id =
            parser::parse_uint(parser::next_rule(&mut pairs, Rule::message_id)?)? as u32;
        let signal_name = parser::next_rule(&mut pairs, Rule::signal_name)?
            .as_str()
            .to_string();
        let multiplexor_name = parser::next_rule(&mut pairs, Rule::multiplexer_name)?
            .as_str()
            .to_string();

        // Collect value pairs
        let mut mappings = Vec::new();
        for pair2 in pairs {
            if pair2.as_rule() == Rule::value_pair {
                let mut min_val = None;
                let mut max_val = None;
                for pairs2 in pair2.into_inner() {
                    if pairs2.as_rule() == Rule::int {
                        let value = parser::parse_uint(pairs2)?;
                        if min_val.is_none() {
                            min_val = Some(value);
                        } else {
                            max_val = Some(value);
                        }
                    }
                }
                if let (Some(min), Some(max)) = (min_val, max_val) {
                    mappings.push(ExtendedMultiplexMapping {
                        min_value: min,
                        max_value: max,
                    });
                }
            }
        }

        let msg_id = if message_id & (1 << 31) != 0 {
            MessageId::Extended(message_id & 0x1FFF_FFFF)
        } else {
            MessageId::Standard(message_id as u16)
        };

        Ok(ExtendedMultiplex {
            message_id: msg_id,
            signal_name,
            multiplexor_signal_name: multiplexor_name,
            mappings,
        })
    }
}
