use can_dbc_pest::{Pair, Rule};

use crate::parser::{expect_empty, next_rule, next_string, parse_uint, validated_inner, DbcError};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnvironmentVariableData {
    pub env_var_name: String,
    pub data_size: u64,
}

impl TryFrom<Pair<'_, Rule>> for EnvironmentVariableData {
    type Error = DbcError;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut pairs = validated_inner(value, Rule::envvar_data)?;

        let env_var_name = next_string(&mut pairs, Rule::env_var_name)?;
        let data_size = parse_uint(next_rule(&mut pairs, Rule::data_size)?)?;
        expect_empty(&pairs)?;

        Ok(Self {
            env_var_name,
            data_size,
        })
    }
}
