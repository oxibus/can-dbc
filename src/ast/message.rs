use can_dbc_pest::{Pair, Rule};

use crate::ast::{MessageId, Signal, Transmitter};
use crate::parser::{expect_empty, next_rule, next_string, parse_uint, single_rule, DbcResult};

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
        let message_id = parse_uint(single_rule(msg_var_pair, Rule::message_id)?)? as u32;
        let name = next_string(&mut pairs, Rule::message_name)?;
        let size = parse_uint(next_rule(&mut pairs, Rule::message_size)?)?;
        let transmitter = next_string(&mut pairs, Rule::transmitter)?;
        expect_empty(&mut pairs)?;

        let transmitter = if matches!(transmitter.as_str(), "Vector__XXX" | "VectorXXX" | "") {
            Transmitter::VectorXXX
        } else {
            Transmitter::NodeName(transmitter)
        };

        Ok(Message {
            id: MessageId::parse(message_id),
            name,
            size,
            transmitter,
            signals: Vec::new(), // Signals will be parsed separately and associated later
        })
    }
}
