//!
//! Parser module for DBC files using pest
//!

use can_dbc_pest::{Error as PestError, Pair, Pairs, Rule};

pub type DbcResult<T> = Result<T, DbcError>;

/// A helper function to decode cp1252 bytes, as DBC files are often encoded in cp1252.
#[cfg(feature = "encodings")]
#[must_use]
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
    #[error("No more rules expected, but found: {0:?}")]
    ExpectedEmpty(Rule),
    #[error("Expected rule: {0:?}, found: {1:?}")]
    ExpectedRule(Rule, Rule),
    #[error("Expected one of these rules: {0:?}, found: {1:?}")]
    ExpectedOneOfRules(Vec<Rule>, Rule),
    #[error("Expected a quoted string or a number, found: {0:?}")]
    ExpectedStrNumber(Rule),
    #[error("Invalid Float value: '{0}'")]
    InvalidFloat(String),
    #[error("Invalid Int value: '{0}'")]
    InvalidInt(String),
    #[error("Invalid Uint value: '{0}'")]
    InvalidUint(String),
    #[error("Message ID out of range: {0}")]
    MessageIdOutOfRange(u64),
    #[error("Multiple multiplexors defined for a message")]
    MultipleMultiplexors,
    #[error("No more parsing rules available")]
    NoMoreRules,
    #[error("Feature not implemented: {0}")]
    NotImplemented(&'static str),
    #[error(transparent)]
    Pest(Box<PestError<Rule>>),
    #[error("Signal defined without an associated message")]
    SignalWithoutMessage,
    #[error("Unknown multiplex indicator: {0}")]
    UnknownMultiplexIndicator(String),
    #[error("Unknown rule: {0:?}")]
    UnknownRule(Rule),
    #[error("Invalid numeric value: '{0}'")]
    InvalidNumericValue(String),
}

impl From<PestError<Rule>> for DbcError {
    fn from(value: PestError<Rule>) -> Self {
        Self::Pest(Box::new(value))
    }
}

/// Helper function to get the next pair and validate its rule
pub(crate) fn next<'a>(iter: &'a mut Pairs<Rule>) -> DbcResult<Pair<'a, Rule>> {
    iter.next().ok_or(DbcError::NoMoreRules)
}

/// Helper function to get the next pair and validate its rule
pub(crate) fn next_rule<'a>(
    iter: &'a mut Pairs<Rule>,
    expected: Rule,
) -> DbcResult<Pair<'a, Rule>> {
    next(iter).and_then(|pair| {
        if pair.as_rule() == expected {
            Ok(pair)
        } else {
            Err(DbcError::ExpectedRule(expected, pair.as_rule()))
        }
    })
}

pub(crate) fn next_optional_rule<'a>(
    iter: &'a mut Pairs<Rule>,
    expected: Rule,
) -> Option<Pair<'a, Rule>> {
    if let Some(pair) = iter.peek() {
        if pair.as_rule() == expected {
            return Some(iter.next().unwrap());
        }
    }
    None
}

/// Helper function to get the next pair, ensure it matches the expected rule, and convert to string
pub(crate) fn next_string(iter: &mut Pairs<Rule>, expected: Rule) -> DbcResult<String> {
    Ok(next_rule(iter, expected)?.as_str().to_string())
}

/// Helper function to get a single pair and validate its rule
pub(crate) fn single_inner(pair: Pair<Rule>, expected: Rule) -> DbcResult<Pair<Rule>> {
    let mut iter = pair.into_inner();
    let pair = iter.next().ok_or(DbcError::NoMoreRules)?;
    if pair.as_rule() != expected {
        Err(DbcError::ExpectedRule(expected, pair.as_rule()))
    } else if let Some(next) = iter.next() {
        Err(DbcError::ExpectedEmpty(next.as_rule()))
    } else {
        Ok(pair)
    }
}

/// Helper function to validate a pair's rule matches the expected rule
pub(crate) fn validated(pair: Pair<Rule>, expected: Rule) -> DbcResult<Pair<Rule>> {
    if pair.as_rule() == expected {
        Ok(pair)
    } else {
        Err(DbcError::ExpectedRule(expected, pair.as_rule()))
    }
}

pub(crate) fn validated_inner(pair: Pair<'_, Rule>, expected: Rule) -> DbcResult<Pairs<'_, Rule>> {
    Ok(validated(pair, expected)?.into_inner())
}

/// Helper function to get a single pair, validate its rule, and convert to string
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
            Err(DbcError::ExpectedRule(expected, pair.as_rule()))
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
            Err(DbcError::ExpectedRule(expected, pair.as_rule()))
        }
    })
    .collect()
}

/// Helper function to ensure the iterator is empty (no more items)
pub(crate) fn expect_empty(iter: &Pairs<Rule>) -> DbcResult<()> {
    iter.peek()
        .map_or(Ok(()), |v| Err(DbcError::ExpectedEmpty(v.as_rule())))
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
pub(crate) fn parse_int(pair: &Pair<Rule>) -> DbcResult<i64> {
    let value = pair.as_str();
    value
        .parse::<i64>()
        .map_err(|_| DbcError::InvalidInt(value.to_string()))
}

/// Helper function to parse an unsigned integer from a pest pair
pub(crate) fn parse_uint(pair: &Pair<Rule>) -> DbcResult<u64> {
    let value = pair.as_str();
    value
        .parse::<u64>()
        .map_err(|_| DbcError::InvalidUint(value.to_string()))
}

/// Helper function to parse a float from a pest pair
pub(crate) fn parse_float(pair: &Pair<Rule>) -> DbcResult<f64> {
    let value = pair.as_str();
    value
        .parse::<f64>()
        .map_err(|_| DbcError::InvalidFloat(value.to_string()))
}

/// Helper function to parse the next uint from the iterator
pub(crate) fn parse_next_uint(iter: &mut Pairs<Rule>, expected: Rule) -> DbcResult<u64> {
    parse_uint(&next_rule(iter, expected)?)
}

/// Helper function to parse the next int from the iterator
pub(crate) fn parse_next_int(iter: &mut Pairs<Rule>, expected: Rule) -> DbcResult<i64> {
    parse_int(&next_rule(iter, expected)?)
}

/// Helper function to parse the next float from the iterator
pub(crate) fn parse_next_float(iter: &mut Pairs<Rule>, expected: Rule) -> DbcResult<f64> {
    parse_float(&next_rule(iter, expected)?)
}

/// Helper function to parse the next string from the iterator
pub(crate) fn parse_next_inner_str(iter: &mut Pairs<Rule>, expected: Rule) -> DbcResult<String> {
    Ok(inner_str(next_rule(iter, expected)?))
}

/// Helper to parse min/max values from a `min_max` rule
pub(crate) fn parse_min_max_int(pair: Pair<Rule>) -> DbcResult<(i64, i64)> {
    let mut pairs = pair.into_inner();

    let min_val = parse_next_int(&mut pairs, Rule::minimum)?;
    let max_val = parse_next_int(&mut pairs, Rule::maximum)?;
    expect_empty(&pairs).expect("pest grammar ensures no extra items");

    Ok((min_val, max_val))
}

/// Helper to parse min/max values from a `min_max` rule as floats
pub(crate) fn parse_min_max_float(pair: Pair<Rule>) -> DbcResult<(f64, f64)> {
    let mut pairs = pair.into_inner();

    let min_val = parse_next_float(&mut pairs, Rule::minimum)?;
    let max_val = parse_next_float(&mut pairs, Rule::maximum)?;
    expect_empty(&pairs).expect("pest grammar ensures no extra items");

    Ok((min_val, max_val))
}
