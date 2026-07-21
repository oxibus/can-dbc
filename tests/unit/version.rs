//! Unit tests for `version`.

use can_dbc::Version;
use can_dbc_pest::Rule;

use crate::common::test_into;

#[test]
fn version_test() {
    let def = r#"
VERSION "HNPBNNNYNNNNNNNNNNNNNNNNNNNNNNNNYNYYYYYYYY>4>%%%/4>'%**4YYY///"
"#;
    let exp = Version("HNPBNNNYNNNNNNNNNNNNNNNNNNNNNNNNYNYYYYYYYY>4>%%%/4>'%**4YYY///".to_string());
    let val = test_into::<Version>(def.trim_start(), Rule::version);
    assert_eq!(val, exp);
}
