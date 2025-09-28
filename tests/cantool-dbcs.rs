#![cfg(feature = "with-serde")]

use insta::{assert_debug_snapshot, assert_yaml_snapshot, with_settings};
use std::fs::File;
use std::io::Read as _;
use std::path::Path;
use std::{fs, io};

/// Test parsing all DBC files in the `tests/cantools-dbcs` directory, using the `can_dbc` crate.
#[test]
fn test_cantools_dbc_files() {
    // Configure insta snapshot content and location
    with_settings! {
        { omit_expression => true,
          snapshot_path => "snapshots-cantools-dbcs",
          prepend_module_to_snapshot => false },
        {
            test_cantools_dbc_files_int().unwrap();
        }
    }
}

/// Test parsing all DBC files in the `tests/cantools-dbcs` directory.
fn test_cantools_dbc_files_int() -> io::Result<()> {
    for dbc_path in fs::read_dir("./tests/cantools-dbcs")? {
        let path = dbc_path?.path();
        if path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.eq_ignore_ascii_case("dbc"))
            .is_some()
        {
            parse_one_file(&path)?;
        }
    }
    Ok(())
}

/// Parse a single DBC file and assert a snapshot of the result.
fn parse_one_file(path: &Path) -> io::Result<()> {
    let file_name = path.file_stem().unwrap().to_string_lossy().to_string();
    eprintln!("Testing DBC file: {file_name}.dbc");
    let mut buffer = Vec::new();
    File::open(path)?.read_to_end(&mut buffer)?;

    match can_dbc::DBC::from_slice(&buffer) {
        Ok(dbc) => assert_yaml_snapshot!(file_name, dbc),
        Err(e) => {
            eprintln!("Failed to parse {file_name}.dbc: {e:?}");
            assert_debug_snapshot!(format!("!error___{file_name}"), e);
        }
    }
    Ok(())
}
