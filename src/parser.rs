//!
//! Parser module for DBC files using pest
//!

use can_dbc_pest::{Error as PestError, Pair, Pairs, Rule};

use crate::encodings;

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

/// Helper function to extract string content from `quoted_str` rule
pub(crate) fn parse_str(pair: Pair<Rule>) -> String {
    if pair.as_rule() == Rule::string {
        return pair.as_str().to_string();
    }
    for pair2 in pair.into_inner() {
        if pair2.as_rule() == Rule::string {
            return pair2.as_str().to_string();
        }
    }
    String::new()
}

/// Helper function to parse an integer from a pest pair
pub(crate) fn parse_int(pair: Pair<Rule>) -> DbcResult<i64> {
    pair.as_str()
        .parse::<i64>()
        .map_err(|_| DbcError::InvalidData)
}

/// Helper function to parse an unsigned integer from a pest pair
pub(crate) fn parse_uint(pair: Pair<Rule>) -> DbcResult<u64> {
    pair.as_str()
        .parse::<u64>()
        .map_err(|_| DbcError::InvalidData)
}

/// Helper function to parse a float from a pest pair
pub(crate) fn parse_float(pair: Pair<Rule>) -> DbcResult<f64> {
    pair.as_str()
        .parse::<f64>()
        .map_err(|_| DbcError::InvalidData)
}

/// Helper function to get the next pair and validate its rule
pub(crate) fn next_rule<'a>(
    iter: &'a mut Pairs<Rule>,
    expected_rule: Rule,
) -> DbcResult<Pair<'a, Rule>> {
    let pair = iter.next().ok_or(DbcError::ParseError)?;
    if pair.as_rule() == expected_rule {
        Ok(pair)
    } else {
        Err(DbcError::ParseError)
    }
}

#[allow(dead_code)]
fn peek_rule<'a>(iter: &mut Pairs<'a, Rule>, expected_rule: Rule) -> Option<Pair<'a, Rule>> {
    if let Some(pair) = iter.peek() {
        if pair.as_rule() == expected_rule {
            return Some(iter.next().unwrap());
        }
    }
    None
}

/// Helper function to ensure the iterator is empty (no more items)
#[allow(dead_code)]
fn expect_empty(iter: &mut Pairs<Rule>) -> DbcResult<()> {
    if iter.next().is_some() {
        Err(DbcError::ParseError)
    } else {
        Ok(())
    }
}

/// Helper to parse min/max values from a `min_max` rule
pub(crate) fn parse_min_max_int(pair: Pair<Rule>) -> DbcResult<(i64, i64)> {
    let mut pairs = pair.into_inner();

    let min_val = parse_int(next_rule(&mut pairs, Rule::minimum)?)?;
    let max_val = parse_int(next_rule(&mut pairs, Rule::maximum)?)?;
    // Don't use expect_empty here as there might be comments or whitespace

    Ok((min_val, max_val))
}

/// Helper to parse min/max values from a `min_max` rule as floats
pub(crate) fn parse_min_max_float(pair: Pair<Rule>) -> DbcResult<(f64, f64)> {
    let mut pairs = pair.into_inner();

    let min_val = parse_float(next_rule(&mut pairs, Rule::minimum)?)?;
    let max_val = parse_float(next_rule(&mut pairs, Rule::maximum)?)?;
    // Don't use expect_empty here as there might be comments or whitespace

    Ok((min_val, max_val))
}
