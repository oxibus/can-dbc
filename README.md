# can-dbc

[![GitHub repo](https://img.shields.io/badge/github-oxibus/can--dbc-8da0cb?logo=github)](https://github.com/oxibus/can-dbc)
[![crates.io version](https://img.shields.io/crates/v/can-dbc)](https://crates.io/crates/can-dbc)
[![crate usage](https://img.shields.io/crates/d/can-dbc)](https://crates.io/crates/can-dbc)
[![docs.rs status](https://img.shields.io/docsrs/can-dbc)](https://docs.rs/can-dbc)
[![crates.io license](https://img.shields.io/crates/l/can-dbc)](https://github.com/oxibus/can-dbc)
[![CI build status](https://github.com/oxibus/can-dbc/actions/workflows/ci.yml/badge.svg)](https://github.com/oxibus/can-dbc/actions)
[![Codecov](https://img.shields.io/codecov/c/github/oxibus/can-dbc)](https://app.codecov.io/gh/oxibus/can-dbc)

A CAN-dbc format parser written with Rust's [Pest grammar](https://pest.rs/) library. CAN databases are used to exchange details about a CAN network, e.g. what messages are being send over the CAN bus and what data do they contain.

## Usage example

Read dbc file and generate Rust structs based on the messages/signals defined in the dbc.

```rust,no_run
use std::fs;
use can_dbc::Dbc;
use codegen::Scope;

fn main() {
    let data = fs::read_to_string("./examples/sample.dbc").expect("Unable to read input file");
    let dbc = Dbc::try_from(data.as_str()).expect("Failed to parse dbc file");

    let mut scope = Scope::new();
    for msg in dbc.messages {
        let msg_struct = scope.new_struct(&msg.name);
        for signal in msg.signals {
            msg_struct.field(&signal.name.to_lowercase(), "f64");
        }
    }

    println!("{}", scope.to_string());
}
```

For a proper implementation for reading or writing CAN frames according to the DBC, I recommend you take a look at [dbc-codegen](https://github.com/technocreatives/dbc-codegen).

## Running Parser Example

The file parser simply parses a dbc input file and prints the parsed content.

```bash
cargo run --example file_parser -- --input examples/sample.dbc
```

# Structure

![Diagram](./docs/diagram.svg)

## Implemented DBC parts

- [x] `version`
- [x] `new_symbols`
- [x] `bit_timing` *(deprecated but mandatory)*
- [x] `nodes`
- [x] `value_tables`
- [x] `messages`
- [x] `message_transmitters`
- [x] `environment_variables`
- [x] `environment_variables_data`
- [x] `signal_types`
- [x] `comments`
- [x] `attribute_definitions`
- [ ] `sigtype_attr_list` *(format missing documentation)*
- [x] `attribute_defaults`
- [x] `attribute_values`
- [x] `value_descriptions`
- [ ] `category_definitions` *(deprecated)*
- [ ] `categories` *(deprecated)*
- [ ] `filter` *(deprecated)*
- [x] `signal_type_refs`
- [x] `signal_groups`
- [x] `signal_extended_value_type_list`

### Deviating from standard
- multispace between parsers instead of single space allowing e.g. (two spaces) `SIG_GROUP  13`.
- `VAL_` suffix may be ` ;` or `;`

## Alternatives
- [canparse](https://github.com/jmagnuson/canparse)

## Credits
Test dbcs files were copied from the [cantools](https://github.com/eerimoq/cantools) project.

## Development
* This project is easier to develop with [just](https://just.systems/man/en/), a modern alternative to `make`.
* To get a list of available commands, run `just`.
* To run tests, use `just test`.
* This project uses [insta](https://insta.rs) for snapshot testing. To update the snapshots run `just bless`

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
  at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual-licensed as above, without any
additional terms or conditions.
