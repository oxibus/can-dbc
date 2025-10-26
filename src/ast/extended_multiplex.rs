use can_dbc_pest::{Pair, Rule};

use crate::ast::{ExtendedMultiplexMapping, MessageId};
use crate::parser::{collect_all, next_rule, next_string, DbcError};

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

impl TryFrom<Pair<'_, Rule>> for ExtendedMultiplex {
    type Error = DbcError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut pairs = pair.into_inner();

        let message_id = next_rule(&mut pairs, Rule::message_id)?.try_into()?;
        let signal_name = next_string(&mut pairs, Rule::signal_name)?;
        let multiplexor_signal_name = next_string(&mut pairs, Rule::multiplexer_name)?;

        // Collect all remaining value pairs
        let mappings: Vec<ExtendedMultiplexMapping> =
            collect_all::<ExtendedMultiplexMapping>(&mut pairs)?;

        Ok(Self {
            message_id,
            signal_name,
            multiplexor_signal_name,
            mappings,
        })
    }
}
