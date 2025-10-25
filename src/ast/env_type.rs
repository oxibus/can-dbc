use crate::DbcError;

/// env_var_type = '0' | '1' | '2' ; (* 0=integer, 1=float, 2=string *)
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EnvType {
    Integer,
    Float,
    String,
}

impl TryFrom<u64> for EnvType {
    type Error = DbcError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Float),
            1 => Ok(Self::Integer),
            2 => Ok(Self::String),
            _ => Err(Self::Error::ParseError),
        }
    }
}
