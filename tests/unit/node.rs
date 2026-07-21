//! Unit tests for `node`.

use can_dbc::Node;
use can_dbc_pest::Rule;

use crate::common::{collect_all, parse};

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
