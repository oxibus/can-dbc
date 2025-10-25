use can_dbc_pest::{Pair, Rule};

use crate::parser::{collect_all, DbcResult};
use crate::DbcError;

/// CAN network nodes, names must be unique
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Node(pub String);

impl Node {
    /// Parse nodes: `BU_: node1 node2 node3 ...`
    pub(crate) fn parse(pair: Pair<Rule>) -> DbcResult<Vec<Self>> {
        collect_all(&mut pair.into_inner())
    }
}

impl TryFrom<Pair<'_, Rule>> for Node {
    type Error = DbcError;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        if pair.as_rule() != Rule::node_name {
            return Err(Self::Error::ParseError);
        }
        Ok(Self(pair.as_str().to_string()))
    }
}
