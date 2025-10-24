# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [7.0.0](https://github.com/oxibus/can-dbc/compare/v6.0.0...v7.0.0) - 2025-10-19

**NOTE**: `can-dbc` has moved to the [OxiBUS](https://github.com/oxibus) GitHub org - a community developing Rust code with focus on communication in automotive and related spaces. If you are maintaining an open source Rust project in this area, consider joining us - it is always easier to maintain projects together!

### Breaking
- major struct and enum naming refactoring. See ([#45](https://github.com/oxibus/can-dbc/pull/45)) for the full list of changes.
- remove `from_slice` - use cp1252 decoder and pass in a `&str` instead ([#44](https://github.com/oxibus/can-dbc/pull/44))
- rename `DBC`→`Dbc` and feature `with-serde`→`serde` ([#42](https://github.com/oxibus/can-dbc/pull/42))

### Added

- added cp1252 decoding support ([#44](https://github.com/oxibus/can-dbc/pull/44))

### Other

- relicense as `MIT OR Apache-2.0` ([#38](https://github.com/oxibus/can-dbc/pull/38))
- update README with usage examples and license information ([#46](https://github.com/oxibus/can-dbc/pull/46))
- move tests to the end ([#43](https://github.com/oxibus/can-dbc/pull/43))
- move test files to submodule, default with serde feature ([#40](https://github.com/oxibus/can-dbc/pull/40))
- allow space after message ID ([#25](https://github.com/oxibus/can-dbc/pull/25))
- upgrade to nom 8 ([#36](https://github.com/oxibus/can-dbc/pull/36))
- clippy lints
- use `insta` to test all parsing results
- use `clap-derive` in example
- run `cargo fmt`
- bump dependencies and minor cleanup
- Simplify tests, test for other escaped characters
- Remove obsolete is_quote function
- Add support for escaped strings within comments
- Derive the Deserialize trait when serde is enabled
- consolidate docs with readme

#### CI improvements
- auto-release and `cargo deny` ([#39](https://github.com/oxibus/can-dbc/pull/39))
- update README to oxibus org ([#37](https://github.com/oxibus/can-dbc/pull/37))
- add automatic validation with precommit on CI side ([#30](https://github.com/oxibus/can-dbc/pull/30))
- modernize CI ([#33](https://github.com/oxibus/can-dbc/pull/33))

## [6.0.0](https://github.com/oxibus/can-dbc/tree/6.0.0) (2024-02-08)
- Update dependencies.
- Breaking: PR #13 and #14. Thank you @erzoe and @kistenklaus.
    Replace u32 in MessageId with enum type to raise awareness that the raw dbc message ids contain the extended bit.
    Add `MessageId.raw()` which returns the previous raw identifier including the extended id!

## [5.0.0](https://github.com/oxibus/can-dbc/tree/5.0.0) (2022-06-22)
- Add support for dbcs with [extended multiplex messages](https://cdn.vector.com/cms/content/know-how/_application-notes/AN-ION-1-0521_Extended_Signal_Multiplexing.pdf). Thank you @pbert519!
- Breaking: `fn message_multiplexor_switch` now returns `Result<Option<&Signal>, Error>` instead of `Option<&Signal>`.
          This is necessary due to the possibility of a message having multiple multiplexer switches.

## [4.0.0](https://github.com/oxibus/can-dbc/tree/4.0.0) (2021-11-15)
- Migrate from nom 4.2 to nom 7.1.
- Allow "random" order of elements in DBC. They still have to be in a block e.g. all messages or comments.
- Remove verbose error in returned error
- Return `&str` of remaining unparsed content instead of `Vec<u8>`.
- Mark `DBC::from_str` as deprecated and replace it with TryFrom::from_str

## [3.0.2](https://github.com/oxibus/can-dbc/tree/3.0.1) (2020-10-28)
- Fix: Allow trailing whitespaces in dbc. Thanks @killercup.

## [3.0.1](https://github.com/oxibus/can-dbc/tree/3.0.1) (2019-05-17)
- Fix: Allow zero time occurence of BU name
- Remove Cargo.lock file

## [3.0.0](https://github.com/oxibus/can-dbc/tree/3.0.0) (2019-11-25)
- BREAKING: Pass `MessageId`'s by value
- Cleanup clippy warnings

## [2.0.0](https://github.com/oxibus/can-dbc/tree/2.0.0) (2019-04-09)
- Change CAN message id type from `u64` to `u32`.
- Update dependencies

## [1.1.0](https://github.com/oxibus/can-dbc/tree/1.1.0) (2019-01-18)
- Add optional feature `with-serde` and derive Serialize for structs and enums.

## [1.0.1](https://github.com/oxibus/can-dbc/tree/1.0.1) (2019-01-15)

### dbcc
- Add first version of dbc to rust compiler

### can-dbc
- Fix plain attribute definition
- Replace singlespace with multispace seperators (less strict)
- Allow multiple signal groups in DBC document
- Accept signal-less messages
- Accept lists in message transmitters
- Lists may now be empty
