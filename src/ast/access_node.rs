use can_dbc_pest::{Pair, Rule};

use crate::DbcError;

// TODO: consider merging with Transmitter

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AccessNode {
    VectorXXX,
    Name(String),
}

impl TryFrom<Pair<'_, Rule>> for AccessNode {
    type Error = DbcError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::node_name => {
                let name = pair.as_str();
                Ok(if name == "VECTOR__XXX" {
                    Self::VectorXXX
                } else {
                    Self::Name(name.to_string())
                })
            }
            _ => Err(Self::Error::ParseError),
        }
    }
}
