use crate::DbcError;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EnvType {
    Float,
    U64,
    Data,
}

impl TryFrom<u64> for EnvType {
    type Error = DbcError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Float),
            1 => Ok(Self::U64),
            2 => Ok(Self::Data),
            _ => Err(Self::Error::ParseError),
        }
    }
}
