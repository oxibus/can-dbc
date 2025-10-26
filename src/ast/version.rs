use can_dbc_pest::{Pair, Rule};

use crate::parser::{inner_str, single_inner, validated, DbcError};

/// Version identifier of the DBC file.
///
/// Format: `VERSION "<VersionIdentifier>"`
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Version(pub String);

impl TryFrom<Pair<'_, Rule>> for Version {
    type Error = DbcError;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let v = single_inner(validated(pair, Rule::version)?, Rule::quoted_str)?;
        Ok(Self(inner_str(v)))
    }
}
