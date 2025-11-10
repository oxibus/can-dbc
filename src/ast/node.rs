use can_dbc_pest::{Pair, Rule};

use crate::parser::{validated, DbcError};

/// CAN network nodes, names must be unique
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Node(pub String);

impl TryFrom<Pair<'_, Rule>> for Node {
    type Error = DbcError;

    fn try_from(value: Pair<Rule>) -> Result<Self, Self::Error> {
        Ok(Self(
            validated(value, Rule::node_name)?.as_str().to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::collect_all;
    use crate::test_helpers::*;

    #[test]
    fn network_node_test() {
        let def = "
BU_: ZU XYZ ABC OIU
";
        let exp = vec![
            Node("ZU".to_string()),
            Node("XYZ".to_string()),
            Node("ABC".to_string()),
            Node("OIU".to_string()),
        ];
        let pair = parse(def.trim_start(), Rule::nodes).unwrap();
        let val: Vec<Node> = collect_all(&mut pair.into_inner()).unwrap();
        assert_eq!(val, exp);
    }

    #[test]
    fn empty_network_node_test() {
        let def = "
BU_:
";
        let pair = parse(def.trim_start(), Rule::nodes).unwrap();
        let val: Vec<Node> = collect_all(&mut pair.into_inner()).unwrap();
        assert_eq!(val, vec![]);
    }
}
