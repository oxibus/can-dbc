use can_dbc_pest::{Pair, Rule};

use crate::ast::{MessageId, Transmitter};
use crate::parser::{collect_expected, next_rule, DbcResult};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageTransmitter {
    pub message_id: MessageId,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub transmitter: Vec<Transmitter>,
}

impl MessageTransmitter {
    /// Parse message transmitter: `BO_TX_BU_ message_id : transmitter1,transmitter2,... ;`
    pub(crate) fn parse(pair: Pair<Rule>) -> DbcResult<Self> {
        let mut pairs = pair.into_inner();

        let message_id = next_rule(&mut pairs, Rule::message_id)?.try_into()?;
        let transmitter = collect_expected(&mut pairs, Rule::transmitter)?;

        Ok(Self {
            message_id,
            transmitter,
        })
    }
}
