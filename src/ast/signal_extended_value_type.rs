use can_dbc_pest::Rule;

use crate::DbcError;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SignalExtendedValueType {
    SignedOrUnsignedInteger,
    IEEEfloat32Bit,
    IEEEdouble64bit,
}

impl TryFrom<Rule> for SignalExtendedValueType {
    type Error = DbcError;

    fn try_from(value: Rule) -> Result<Self, Self::Error> {
        match value {
            Rule::sig_val_integer => Ok(Self::SignedOrUnsignedInteger),
            Rule::sig_val_IEEE_float_32Bit => Ok(Self::IEEEfloat32Bit),
            Rule::sig_val_IEEE_float_64Bit => Ok(Self::IEEEdouble64bit),
            v => Err(DbcError::UnknownRule(v)),
        }
    }
}
