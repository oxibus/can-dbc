//!
//! Parser module for DBC files using pest
//!

use can_dbc_pest::{Error as PestError, Pair, Pairs, Rule};

pub type DbcResult<T> = Result<T, DbcError>;

/// A helper function to decode cp1252 bytes, as DBC files are often encoded in cp1252.
#[cfg(feature = "encodings")]
pub fn decode_cp1252(bytes: &[u8]) -> Option<std::borrow::Cow<'_, str>> {
    let (cow, _, had_errors) = crate::encodings::WINDOWS_1252.decode(bytes);
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
    #[error("Expected rule: {0:?}, found: {1:?}")]
    Expected(Rule, Rule),
    #[error("Expected a quoted string or a number, found: {0:?}")]
    ExpectedNumber(Rule),
    #[error("Unknown rule: {0:?}")]
    UnknownRule(Rule),
    #[error("No more parsing rules available")]
    NoMoreRules,
    #[error("No more rules expected, but found: {0:?}")]
    UnexpectedRule(Rule),
    #[error("Signal defined without an associated message")]
    SignalWithoutMessage,
}

impl From<PestError<Rule>> for DbcError {
    fn from(value: PestError<Rule>) -> Self {
        Self::Pest(Box::new(value))
    }
}

/// Helper function to get the next pair and validate its rule
pub(crate) fn next<'a>(iter: &'a mut Pairs<Rule>) -> DbcResult<Pair<'a, Rule>> {
    iter.next().ok_or(DbcError::ParseError)
}

/// Helper function to get the next pair and validate its rule
pub(crate) fn next_rule<'a>(
    iter: &'a mut Pairs<Rule>,
    expected: Rule,
) -> DbcResult<Pair<'a, Rule>> {
    iter.next().ok_or(DbcError::ParseError).and_then(|pair| {
        if pair.as_rule() == expected {
            Ok(pair)
        } else {
            Err(DbcError::ParseError)
        }
    })
}

#[allow(dead_code)]
pub(crate) fn next_optional_rule<'a>(
    iter: &'a mut Pairs<Rule>,
    expected: Rule,
) -> DbcResult<Option<Pair<'a, Rule>>> {
    if let Some(pair) = iter.peek() {
        if pair.as_rule() == expected {
            return Ok(Some(iter.next().unwrap()));
        }
    }
    Ok(None)
}

/// Helper function to get the next pair, ensure it matches the expected rule, and convert to string
pub(crate) fn next_string(iter: &mut Pairs<Rule>, expected: Rule) -> DbcResult<String> {
    Ok(next_rule(iter, expected)?.as_str().to_string())
}

/// Helper function to get a single pair and validate its rule
pub(crate) fn single_inner(pair: Pair<Rule>, expected: Rule) -> DbcResult<Pair<Rule>> {
    let mut iter = pair.into_inner();
    let pair = iter.next().ok_or(DbcError::ParseError)?;
    if pair.as_rule() != expected {
        Err(DbcError::Expected(expected, pair.as_rule()))
    } else if iter.next().is_some() {
        Err(DbcError::ParseError)
    } else {
        Ok(pair)
    }
}

/// Helper function to validate a pair's rule matches the expected rule
pub(crate) fn validated(pair: Pair<Rule>, expected: Rule) -> DbcResult<Pair<Rule>> {
    if pair.as_rule() == expected {
        Ok(pair)
    } else {
        Err(DbcError::Expected(expected, pair.as_rule()))
    }
}

pub(crate) fn validated_inner(pair: Pair<'_, Rule>, expected: Rule) -> DbcResult<Pairs<'_, Rule>> {
    Ok(validated(pair, expected)?.into_inner())
}

/// Helper function to get a single pair, validate its rule, and convert to string
#[allow(dead_code)]
pub(crate) fn single_inner_str(pair: Pair<Rule>, expected: Rule) -> DbcResult<String> {
    Ok(single_inner(pair, expected)?.as_str().to_string())
}

/// Helper function to collect all remaining pairs of a specific rule type
pub(crate) fn collect_all<'a, T: TryFrom<Pair<'a, Rule>, Error = DbcError>>(
    iter: &mut Pairs<'a, Rule>,
) -> DbcResult<Vec<T>> {
    iter.map(TryInto::try_into).collect()
}

/// Helper function to collect all remaining pairs of a specific rule type
pub(crate) fn collect_expected<'a, T: TryFrom<Pair<'a, Rule>, Error = DbcError>>(
    iter: &mut Pairs<'a, Rule>,
    expected: Rule,
) -> DbcResult<Vec<T>> {
    iter.map(|pair| {
        if pair.as_rule() == expected {
            pair.try_into()
        } else {
            Err(DbcError::ParseError)
        }
    })
    .collect()
}

/// Helper function to collect all remaining pairs of a specific rule type and convert to strings
pub(crate) fn collect_strings(iter: &mut Pairs<Rule>, expected: Rule) -> DbcResult<Vec<String>> {
    iter.map(|pair| {
        if pair.as_rule() == expected {
            Ok(pair.as_str().to_string())
        } else {
            Err(DbcError::ParseError)
        }
    })
    .collect()
}

/// Helper function to ensure the iterator is empty (no more items)
pub(crate) fn expect_empty(iter: &Pairs<Rule>) -> DbcResult<()> {
    iter.peek().map_or(Ok(()), |_| Err(DbcError::ParseError))
}

/// Helper function to extract string content from `quoted_str` rule
pub(crate) fn inner_str(pair: Pair<Rule>) -> String {
    // panics because pest grammar ensures this
    next_rule(&mut pair.into_inner(), Rule::string)
        .expect("string")
        .as_str()
        .to_string()
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

/// Helper to parse min/max values from a `min_max` rule
pub(crate) fn parse_min_max_int(pair: Pair<Rule>) -> DbcResult<(i64, i64)> {
    let mut pairs = pair.into_inner();

    let min_val = parse_int(next_rule(&mut pairs, Rule::minimum)?)?;
    let max_val = parse_int(next_rule(&mut pairs, Rule::maximum)?)?;
    expect_empty(&pairs).expect("pest grammar ensures no extra items");

    Ok((min_val, max_val))
}

/// Helper to parse min/max values from a `min_max` rule as floats
pub(crate) fn parse_min_max_float(pair: Pair<Rule>) -> DbcResult<(f64, f64)> {
    let mut pairs = pair.into_inner();

    let min_val = parse_float(next_rule(&mut pairs, Rule::minimum)?)?;
    let max_val = parse_float(next_rule(&mut pairs, Rule::maximum)?)?;
    expect_empty(&pairs).expect("pest grammar ensures no extra items");

    Ok((min_val, max_val))
}
