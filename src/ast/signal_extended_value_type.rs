use crate::DbcError;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SignalExtendedValueType {
    SignedOrUnsignedInteger,
    IEEEfloat32Bit,
    IEEEdouble64bit,
}

impl TryFrom<u64> for SignalExtendedValueType {
    type Error = DbcError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SignedOrUnsignedInteger),
            1 => Ok(Self::IEEEfloat32Bit),
            2 => Ok(Self::IEEEdouble64bit),
            _ => Err(Self::Error::ParseError),
        }
    }
}
