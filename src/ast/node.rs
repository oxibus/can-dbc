use can_dbc_pest::{Pair, Rule};

use crate::parser::{validated, DbcError};

/// CAN network nodes, names must be unique
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Node(pub String);

impl TryFrom<Pair<'_, Rule>> for Node {
    type Error = DbcError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        Ok(Self(validated(pair, Rule::node_name)?.as_str().to_string()))
    }
}
