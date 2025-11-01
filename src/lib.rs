#![doc = include_str!("../README.md")]

mod ast;
mod extend;
mod parser;

#[cfg(test)]
mod parser_tests;

// Re-export all types from the ast module
pub use ast::*;
// Re-export of `encoding_rs` as encodings to simplify usage
#[cfg(feature = "encodings")]
pub use encoding_rs as encodings;
pub use extend::*;
#[cfg(feature = "encodings")]
pub use parser::decode_cp1252;
pub use parser::{DbcError, DbcResult};
