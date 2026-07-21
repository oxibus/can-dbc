//! Shared helpers for AST / pest rule unit tests.

use std::fmt::Debug;

use can_dbc::DbcResult;
use can_dbc_pest::{DbcParser, Pair, Pairs, Parser, Rule};

/// Parse `input` with a specific pest `rule`, returning the first matched pair.
pub fn parse(input: &str, rule: Rule) -> DbcResult<Pair<'_, Rule>> {
    let pairs = DbcParser::parse(rule, input)?;
    Ok(pairs
        .into_iter()
        .next()
        .expect("parser returned no pairs for a successful parse"))
}

/// Parse `input` and return the matched span as a string slice.
pub fn span(input: &str, rule: Rule) -> &str {
    parse(input, rule)
        .unwrap_or_else(|e| panic!("parse failed for {input:?}: {e:?}"))
        .as_span()
        .as_str()
}

/// Parse `input` with `rule` and convert the pair into `T`.
pub fn test_into<'a, T>(input: &'a str, rule: Rule) -> T
where
    T: TryFrom<Pair<'a, Rule>>,
    <T as TryFrom<Pair<'a, Rule>>>::Error: Debug,
{
    let pair = parse(input, rule).unwrap_or_else(|e| {
        panic!("Parse {e:?}:\n{input:#?}");
    });
    pair.clone().try_into().unwrap_or_else(|e| {
        panic!("Into {e:?}:\n{pair:#?}");
    })
}

/// Convert every remaining inner pair into `T`.
pub fn collect_all<'a, T>(pairs: &mut Pairs<'a, Rule>) -> DbcResult<Vec<T>>
where
    T: TryFrom<Pair<'a, Rule>, Error = can_dbc::DbcError>,
{
    pairs.map(TryInto::try_into).collect()
}
