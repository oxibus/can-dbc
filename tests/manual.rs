#![cfg(feature = "serde")]

use std::fs;
use std::path::Path;

use can_dbc::{decode_cp1252, Dbc};
use insta::assert_yaml_snapshot;

#[test]
#[ignore = "manual test for debugging purposes"]
fn manual_test() {
    let path = Path::new("tests/fixtures/opendbc/opendbc/dbc/generator/honda/_honda_common.dbc");
    let buffer = fs::read(path).unwrap();
    if let Some(buffer) = decode_cp1252(&buffer) {
        let buffer = Dbc::try_from(buffer.as_ref()).unwrap_or_else(|e| {
            panic!("{e:?}:\n{buffer:#?}");
        });
        assert_yaml_snapshot!(buffer, @r#"
        ...
        "#);
    }
}
