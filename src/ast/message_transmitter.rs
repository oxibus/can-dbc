use can_dbc_pest::{Pair, Rule};

use crate::ast::{MessageId, Transmitter};
use crate::parser::{next_rule, parse_uint, DbcResult};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageTransmitter {
    pub message_id: MessageId,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub transmitter: Vec<Transmitter>,
}

impl MessageTransmitter {
    /// Parse message transmitter: `BO_TX_BU_ message_id : transmitter1,transmitter2,... ;`
    pub(crate) fn parse(pair: Pair<Rule>) -> DbcResult<MessageTransmitter> {
        let mut pairs = pair.into_inner();

        let message_id = parse_uint(next_rule(&mut pairs, Rule::message_id)?)? as u32;

        let mut transmitter = Vec::new();
        for pair2 in pairs {
            if pair2.as_rule() == Rule::transmitter {
                let name = pair2.as_str().to_string();
                transmitter.push(if name == "Vector__XXX" {
                    Transmitter::VectorXXX
                } else {
                    Transmitter::NodeName(name)
                });
            }
        }

        Ok(MessageTransmitter {
            message_id: MessageId::parse(message_id),
            transmitter,
        })
    }
}
