use can_dbc_pest::{Pair, Rule};

use crate::DbcError;

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
        match pair.as_rule() {
            Rule::transmitter => {
                let name = pair.as_str();
                Ok(if name == "Vector__XXX" {
                    Self::VectorXXX
                } else {
                    Self::NodeName(name.to_string())
                })
            }
            _ => Err(Self::Error::ParseError),
        }
    }
}
