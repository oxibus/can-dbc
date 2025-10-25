use std::str;

use crate::parser::DbcError;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MultiplexIndicator {
    /// Multiplexor switch
    Multiplexor,
    /// Signal is being multiplexed by the multiplexer switch.
    MultiplexedSignal(u64),
    /// Signal is being multiplexed by the multiplexer switch and itself is a multiplexer
    MultiplexorAndMultiplexedSignal(u64),
    /// Normal signal
    Plain,
}

impl TryFrom<&str> for MultiplexIndicator {
    type Error = DbcError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        if text == "M" {
            return Ok(Self::Multiplexor);
        }
        if let Some(text) = text.strip_prefix('m') {
            // Multiplexed signal value should be like "m1" or "m1M"
            // Check if it ends with 'M' (multiplexer and multiplexed signal)
            if text.is_empty() {
                // FIXME: is this the right interpretation?
                return Ok(Self::Plain);
            } else if let Some(text) = text.strip_suffix('M') {
                if let Ok(value) = text.parse::<u64>() {
                    return Ok(Self::MultiplexorAndMultiplexedSignal(value));
                }
            } else if let Ok(value) = text.parse::<u64>() {
                return Ok(Self::MultiplexedSignal(value));
            }
        }

        Err(Self::Error::ParseError)
    }
}
