use can_dbc_pest::{Pair, Rule};

use crate::parser::{validated, DbcError};

// TODO: consider merging with AccessNode

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Transmitter {
    /// node transmitting the message
    NodeName(String),
    /// message has no sender
    VectorXXX,
}

impl TryFrom<Pair<'_, Rule>> for Transmitter {
    type Error = DbcError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let value = validated(pair, Rule::transmitter)?.as_str();
        Ok(if value == "Vector__XXX" {
            Self::VectorXXX
        } else {
            Self::NodeName(value.to_string())
        })
    }
}
