use can_dbc_pest::{Pair, Rule};

use crate::parser::{validated, DbcError};

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
        let value = validated(pair, Rule::node_name)?.as_str();
        Ok(if value == "VECTOR__XXX" {
            Self::VectorXXX
        } else {
            Self::Name(value.to_string())
        })
    }
}
