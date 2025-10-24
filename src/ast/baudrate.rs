use can_dbc_pest::{Pair, Rule};

use crate::parser::DbcResult;

/// Baudrate of network in kbit/s
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Baudrate(pub u64);

impl Baudrate {
    /// Parse bit timing: BS_: `[baud_rate : BTR1 , BTR2 ]`
    pub(crate) fn parse_bit_timing(pair: Pair<Rule>) -> DbcResult<Vec<Baudrate>> {
        let pairs = pair.into_inner();
        if pairs.len() == 0 {
            return Ok(vec![]);
        }
        todo!("Bit timing parsing not implemented yet");
    }
}
