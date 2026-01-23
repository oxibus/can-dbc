use can_dbc_pest::Rule;

use crate::DbcError;

/// `env_var_type = '0' | '1' | '2' ; (* 0=integer, 1=float, 2=string *)`
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EnvType {
    Integer,
    Float,
    String,
}

impl TryFrom<Rule> for EnvType {
    type Error = DbcError;

    fn try_from(value: Rule) -> Result<Self, Self::Error> {
        match value {
            Rule::env_var_type_int => Ok(Self::Integer),
            Rule::env_var_type_float => Ok(Self::Float),
            Rule::env_var_type_string => Ok(Self::String),
            v => Err(DbcError::UnknownRule(v)),
        }
    }
}
