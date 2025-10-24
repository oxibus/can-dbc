use can_dbc_pest::{Pair, Rule};

use crate::parser::DbcResult;

/// CAN network nodes, names must be unique
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Node(pub String);

impl Node {
    /// Parse nodes: BU_: node1 node2 node3 ...
    pub(crate) fn parse_nodes(pair: Pair<Rule>) -> DbcResult<Vec<Node>> {
        let mut nodes = Vec::new();

        for pair2 in pair.into_inner() {
            if let Rule::node_name = pair2.as_rule() {
                nodes.push(Node(pair2.as_str().to_string()));
            }
        }

        Ok(nodes)
    }
}
