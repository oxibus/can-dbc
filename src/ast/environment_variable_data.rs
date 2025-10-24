use can_dbc_pest::{Pair, Rule};

use crate::{parser, DbcResult};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnvironmentVariableData {
    pub env_var_name: String,
    pub data_size: u64,
}

impl EnvironmentVariableData {
    pub(crate) fn parse(pair: Pair<Rule>) -> DbcResult<EnvironmentVariableData> {
        let mut inner_pairs = pair.into_inner();

        let variable_name = parser::next_rule(&mut inner_pairs, Rule::env_var_name)?
            .as_str()
            .to_string();
        let data_size = parser::parse_uint(parser::next_rule(&mut inner_pairs, Rule::data_size)?)?;

        // Don't use expect_empty here as there might be comments or whitespace

        Ok(EnvironmentVariableData {
            env_var_name: variable_name,
            data_size,
        })
    }
}
