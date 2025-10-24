use can_dbc_pest::{Pair, Rule};

use crate::parser::{expect_empty, opt_rule, DbcResult};

/// CAN network nodes, names must be unique
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Node(pub String);

impl Node {
    /// Parse nodes: `BU_: node1 node2 node3 ...`
    pub(crate) fn parse_nodes(pair: Pair<Rule>) -> DbcResult<Vec<Node>> {
        let mut nodes = Vec::new();

        let mut pairs = pair.into_inner();
        while let Some(pair) = opt_rule(&mut pairs, Rule::node_name) {
            nodes.push(Node(pair.as_str().to_string()));
        }
        expect_empty(&mut pairs)?;

        Ok(nodes)
    }
}
