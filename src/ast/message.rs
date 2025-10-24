use can_dbc_pest::{Pair, Rule};

use crate::ast::{MessageId, Signal, Transmitter};
use crate::parser::{expect_empty, next_rule, parse_uint, DbcResult};

/// CAN message (frame) details including signal details
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Message {
    /// CAN id in header of CAN frame.
    /// Must be unique in DBC file.
    pub id: MessageId,
    pub name: String,
    pub size: u64,
    pub transmitter: Transmitter,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub signals: Vec<Signal>,
}

impl Message {
    /// Parse message: `BO_ message_id message_name: message_size transmitter`
    pub(crate) fn parse(pair: Pair<Rule>) -> DbcResult<Message> {
        let mut pairs = pair.into_inner();

        // Parse msg_var (contains msg_literal ~ message_id)
        let msg_var_pair = next_rule(&mut pairs, Rule::msg_var)?;
        let mut message_id = 0u32;
        for sub_pair in msg_var_pair.into_inner() {
            if sub_pair.as_rule() == Rule::message_id {
                message_id = parse_uint(sub_pair)? as u32;
            }
        }

        let message_name = next_rule(&mut pairs, Rule::message_name)?
            .as_str()
            .to_string();
        let message_size = parse_uint(next_rule(&mut pairs, Rule::message_size)?)?;
        let transmitter = next_rule(&mut pairs, Rule::transmitter)?
            .as_str()
            .to_string();
        expect_empty(&mut pairs)?;

        let msg_id = if message_id & (1 << 31) != 0 {
            MessageId::Extended(message_id & 0x1FFF_FFFF)
        } else {
            MessageId::Standard(message_id as u16)
        };

        let transmitter =
            if transmitter == "Vector__XXX" || transmitter == "VectorXXX" || transmitter.is_empty()
            {
                Transmitter::VectorXXX
            } else {
                Transmitter::NodeName(transmitter)
            };

        Ok(Message {
            id: msg_id,
            name: message_name,
            size: message_size,
            transmitter,
            signals: Vec::new(), // Signals will be parsed separately and associated later
        })
    }
}
