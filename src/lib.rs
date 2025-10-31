#![doc = include_str!("../README.md")]

mod ast;
mod parser;

use std::convert::TryFrom;

use derive_getters::Getters;

mod extend;
pub mod parser;
pub use extend::*;
// Re-export all types from the ast module
pub use ast::*;

#[cfg(test)]
mod parser_tests;

/// Re-export of `encoding_rs` as encodings to simplify usage
#[cfg(feature = "encodings")]
pub use encoding_rs as encodings;

/// A helper function to decode cp1252 bytes, as DBC files are often encoded in cp1252.
#[cfg(feature = "encodings")]
pub fn decode_cp1252(bytes: &[u8]) -> Option<std::borrow::Cow<'_, str>> {
    let (cow, _, had_errors) = encodings::WINDOWS_1252.decode(bytes);
    if had_errors {
        None
    } else {
        Some(cow)
    }
}
