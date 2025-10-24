#![doc = include_str!("../README.md")]

mod ast;
mod parser;
#[cfg(test)]
mod parser_tests;

// Re-export all types from the ast module
pub use ast::*;
use can_dbc_pest::{Error as PestError, Rule};
/// Re-export of `encoding_rs` as encodings to simplify usage
#[cfg(feature = "encodings")]
pub use encoding_rs as encodings;

pub type DbcResult<T> = Result<T, DbcError>;

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

/// Error type for DBC parsing operations
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum DbcError {
    #[error(transparent)]
    Pest(Box<PestError<Rule>>),
    #[error("Invalid data")]
    InvalidData,
    #[error("Unknown parse error")]
    ParseError,
    #[error("Multiple multiplexors defined for a message")]
    MultipleMultiplexors,
    #[error("Feature not implemented: {0}")]
    NotImplemented(&'static str),
}

impl From<PestError<Rule>> for DbcError {
    fn from(value: PestError<Rule>) -> Self {
        Self::Pest(Box::new(value))
    }
}
