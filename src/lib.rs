#![doc = include_str!("../README.md")]

// Re-export of `can_dbc_pest::encodings` to simplify usage
#[cfg(feature = "encodings")]
pub use can_dbc_pest::{decode_cp1252, encodings};

mod ast;
pub use ast::*;

mod parser;
pub use parser::{DbcError, DbcResult};
