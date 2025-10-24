use can_dbc_pest::{Pair, Rule};

use crate::parser::{expect_empty, next_rule, parse_str, DbcResult};

/// Version identifier of the DBC file.
///
/// Format: `VERSION "<VersionIdentifier>"`
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Version(pub String);

impl Version {
    /// Parse version: VERSION "string"
    pub(crate) fn parse(pair: Pair<Rule>) -> DbcResult<Version> {
        let mut pairs = pair.into_inner();
        let version_str = parse_str(next_rule(&mut pairs, Rule::quoted_str)?);
        expect_empty(&mut pairs)?;

        Ok(Version(version_str))
    }
}
