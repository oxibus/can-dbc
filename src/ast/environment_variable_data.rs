use can_dbc_pest::{Pair, Rule};

use crate::parser::{expect_empty, next_string, parse_next_uint, validated_inner, DbcError};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnvironmentVariableData {
    pub env_var_name: String,
    pub data_size: u64,
}

impl TryFrom<Pair<'_, Rule>> for EnvironmentVariableData {
    type Error = DbcError;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut pairs = validated_inner(value, Rule::env_var_data)?;

        let env_var_name = next_string(&mut pairs, Rule::env_var_name)?;
        let data_size = parse_next_uint(&mut pairs, Rule::data_size)?;
        expect_empty(&pairs)?;

        Ok(Self {
            env_var_name,
            data_size,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

    #[test]
    fn envvar_data_test() {
        let def = "
ENVVAR_DATA_ SomeEnvVarData: 399;
";
        let exp = EnvironmentVariableData {
            env_var_name: "SomeEnvVarData".to_string(),
            data_size: 399,
        };
        let val = test_into::<EnvironmentVariableData>(def.trim_start(), Rule::env_var_data);
        assert_eq!(val, exp);
    }
}
