//! Unit tests for `symbol`.

use can_dbc::Symbol;
use can_dbc_pest::Rule;

use crate::common::{collect_all, parse};

#[test]
fn new_symbols_test() {
    let def = "
NS_ :
    NS_DESC_
    CM_
    BA_DEF_
";
    let exp = vec![
        Symbol("NS_DESC_".to_string()),
        Symbol("CM_".to_string()),
        Symbol("BA_DEF_".to_string()),
    ];
    let pair = parse(def.trim_start(), Rule::new_symbols).unwrap();
    let mut pairs = pair.into_inner();
    let val: Vec<Symbol> = collect_all(&mut pairs).unwrap();
    assert_eq!(val, exp);
}
