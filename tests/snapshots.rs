#![cfg(all(feature = "serde", feature = "encodings"))]

use std::borrow::Cow;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::{env, fs};

use can_dbc::{decode_cp1252, Dbc};
use insta::{assert_debug_snapshot, assert_yaml_snapshot, with_settings};
#[cfg(windows)]
use path_slash::PathBufExt;
use test_each_file::test_each_path;

struct TestConfig {
    test_root: &'static str,
    snapshot_suffix: &'static str,
    use_cp1251: bool,
    create_snapshot: bool,
}

static TEST_DIRS: &[TestConfig] = &[
    TestConfig {
        test_root: "shared-test-files",
        snapshot_suffix: "",
        use_cp1251: true,
        create_snapshot: true,
    },
    TestConfig {
        test_root: "opendbc/opendbc/dbc",
        snapshot_suffix: "opendbc",
        use_cp1251: false,
        create_snapshot: false,
    },
];

test_each_path! { for ["dbc"] in "./tests/fixtures/opendbc/opendbc/dbc" as dbc => parse_one_file }
test_each_path! { for ["dbc"] in "./tests/fixtures/shared-test-files" as shared => parse_one_file }
// upper case extension
test_each_path! { for ["DBC"] in "./tests/fixtures/shared-test-files" as shared2 => parse_one_file }

struct Test {
    config: &'static TestConfig,
    path: PathBuf,
    file_name: String,
}

impl Test {
    fn new(config: &'static TestConfig, path: PathBuf, file_name: String) -> Self {
        Self {
            config,
            path,
            file_name,
        }
    }
    fn decode<'a>(&self, data: &'a [u8]) -> Cow<'a, str> {
        if self.config.use_cp1251 {
            decode_cp1252(data)
                .unwrap_or_else(|| panic!("Cannot decode {} as cp1252", self.path.display()))
        } else {
            std::str::from_utf8(data)
                .unwrap_or_else(|_| panic!("Cannot decode {} as utf-8", self.path.display()))
                .into()
        }
    }
    fn snapshot_path(&self, is_error: bool) -> Option<PathBuf> {
        (if is_error || self.config.create_snapshot {
            Some("snapshots")
        } else if env::var("FORCE_INSTA").is_ok() {
            Some("snapshots-forced") // this dir is .gitignored
        } else {
            None
        })
        .map(|v| {
            PathBuf::from(v)
                .join(self.config.snapshot_suffix)
                .join(&self.path)
        })
    }
    fn file_name(&self, is_error: bool) -> String {
        if is_error {
            format!("!error___{}", self.file_name)
        } else {
            self.file_name.clone()
        }
    }
}

/// Get snapshot path (if snapshot should be created) and a decoding
/// function for a test file path
fn get_test_info(path: &Path) -> Test {
    let path_str = path.display().to_string();
    let parent = path.parent().unwrap();
    for item in TEST_DIRS {
        // Ensure slashes are there for easier matching
        // Note that all slashes are in Linux style, automatically converted on Windows
        let test_root = format!("/{}/", item.test_root);
        let mut path_dir = parent.to_str().unwrap().to_string();
        if !path_dir.ends_with('/') {
            path_dir.push('/');
        }
        // For windows, convert to slashes for easier matching
        #[cfg(windows)]
        let (test_root, path_dir) = (
            PathBuf::from_slash(&test_root)
                .to_str()
                .unwrap()
                .to_string(),
            PathBuf::from_slash(&path_dir).to_str().unwrap().to_string(),
        );
        if let Some(pos) = path_dir.find(&test_root) {
            let file_name = path.file_stem().unwrap().to_string_lossy().to_string();
            let path = PathBuf::from(&path_dir[pos + test_root.len()..]);
            return Test::new(item, path, file_name);
        }
    }
    panic!("Unknown test directory: {path_str}");
}

/// Test parsing all DBC files
#[test]
fn test_if_submodules_are_present() {
    for test in TEST_DIRS {
        let dir = Path::new("./tests/fixtures").join(test.test_root);
        fs::read_dir(&dir)
            .and_then(|v| {
                v.into_iter()
                    .next()
                    .map(|_| ())
                    .ok_or_else(|| Error::new(ErrorKind::NotFound, "No files or dirs found"))
            })
            .unwrap_or_else(|e| {
                let dir_display = dir.display();
                panic!(
                    "
--------------------------------------------------------------------------
Error reading dbc test files from   {dir_display}
{e}
Make sure git submodules are up to date by running
    git submodule update --init --recursive
--------------------------------------------------------------------------
"
                )
            });
    }
}

/// Parse a single DBC file and assert a snapshot of the result.
fn parse_one_file([path]: [&Path; 1]) {
    let test = get_test_info(path);
    let buffer = fs::read(path).unwrap();
    let buffer = test.decode(&buffer);
    let result = Dbc::try_from(buffer.as_ref());
    let is_err = result.is_err();

    if let Some(snapshot_path) = test.snapshot_path(is_err) {
        with_settings! {
            {
                omit_expression => true,
                prepend_module_to_snapshot => false,
                snapshot_path => snapshot_path,
            },
            {
                match result {
                    Ok(v) => assert_yaml_snapshot!(test.file_name(is_err), v),
                    Err(e) => assert_debug_snapshot!(test.file_name(is_err), e.to_string()),
                }
            }
        }
    } else if let Err(e) = result {
        panic!("Failed to parse {}.dbc: {e:#?}", test.file_name);
    }
}
