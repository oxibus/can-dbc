use can_dbc_pest::{Pair, Rule};

use crate::ast::{MessageId, Transmitter};
use crate::{parser, DbcResult};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageTransmitter {
    pub message_id: MessageId,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub transmitter: Vec<Transmitter>,
}

/// Parse message transmitter: `BO_TX_BU_ message_id : transmitter1,transmitter2,... ;`
pub(crate) fn parse_message_transmitter(pair: Pair<Rule>) -> DbcResult<MessageTransmitter> {
    let mut inner_pairs = pair.into_inner();

    let message_id =
        parser::parse_uint(parser::next_rule(&mut inner_pairs, Rule::message_id)?)? as u32;

    // Collect transmitters
    let mut transmitters = Vec::new();
    for pair2 in inner_pairs {
        if pair2.as_rule() == Rule::transmitter {
            let name = pair2.as_str().to_string();
            let transmitter = if name == "Vector__XXX" {
                Transmitter::VectorXXX
            } else {
                Transmitter::NodeName(name)
            };
            transmitters.push(transmitter);
        }
    }

    let msg_id = if message_id & (1 << 31) != 0 {
        MessageId::Extended(message_id & 0x1FFF_FFFF)
    } else {
        MessageId::Standard(message_id as u16)
    };

    Ok(MessageTransmitter {
        message_id: msg_id,
        transmitter: transmitters,
    })
}
