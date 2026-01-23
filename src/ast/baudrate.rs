use can_dbc_pest::{Pair, Rule};

use crate::parser::DbcError;

/// Baudrate of network in KBit/s
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Baudrate(pub u64);

impl TryFrom<Pair<'_, Rule>> for Baudrate {
    type Error = DbcError;

    /// Parse bit timing: `BS_: [baud_rate : BTR1 , BTR2 ]`
    fn try_from(_value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        todo!("Bit timing parsing not implemented yet");
    }
}
