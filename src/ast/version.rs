use can_dbc_pest::{Pair, Rule};

use crate::parser::{inner_str, single_inner, validated, DbcError};

/// Version identifier of the DBC file.
///
/// Format: `VERSION "<VersionIdentifier>"`
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Version(pub String);

impl TryFrom<Pair<'_, Rule>> for Version {
    type Error = DbcError;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let v = single_inner(validated(value, Rule::version)?, Rule::quoted_str)?;
        Ok(Self(inner_str(v)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

    #[test]
    fn version_test() {
        let def = r#"
VERSION "HNPBNNNYNNNNNNNNNNNNNNNNNNNNNNNNYNYYYYYYYY>4>%%%/4>'%**4YYY///"
"#;
        let exp =
            Version("HNPBNNNYNNNNNNNNNNNNNNNNNNNNNNNNYNYYYYYYYY>4>%%%/4>'%**4YYY///".to_string());
        let val = test_into::<Version>(def.trim_start(), Rule::version);
        assert_eq!(val, exp);
    }
}
