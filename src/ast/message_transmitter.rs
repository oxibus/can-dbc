use can_dbc_pest::{Pair, Rule};

use crate::ast::{MessageId, Transmitter};
use crate::parser::{collect_expected, next_rule, validated_inner, DbcError};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageTransmitter {
    pub message_id: MessageId,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub transmitter: Vec<Transmitter>,
}

impl TryFrom<Pair<'_, Rule>> for MessageTransmitter {
    type Error = DbcError;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut pairs = validated_inner(value, Rule::bo_tx_bu)?;
        let message_id = next_rule(&mut pairs, Rule::message_id)?.try_into()?;
        let transmitter = collect_expected(&mut pairs, Rule::transmitter)?;

        Ok(Self {
            message_id,
            transmitter,
        })
    }
}
