use can_dbc_pest::{Pair, Rule};

use crate::parser::{expect_empty, next_rule, parse_uint, DbcResult};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnvironmentVariableData {
    pub env_var_name: String,
    pub data_size: u64,
}

impl EnvironmentVariableData {
    pub(crate) fn parse(pair: Pair<Rule>) -> DbcResult<EnvironmentVariableData> {
        let mut pairs = pair.into_inner();

        let variable_name = next_rule(&mut pairs, Rule::env_var_name)?
            .as_str()
            .to_string();
        let data_size = parse_uint(next_rule(&mut pairs, Rule::data_size)?)?;
        expect_empty(&mut pairs)?;

        Ok(EnvironmentVariableData {
            env_var_name: variable_name,
            data_size,
        })
    }
}
