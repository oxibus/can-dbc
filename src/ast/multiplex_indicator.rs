use std::str;

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MultiplexIndicator {
    /// Multiplexor switch
    Multiplexor,
    /// Signal is being multiplexed by the multiplexer switch.
    MultiplexedSignal(u64),
    /// Signal is being multiplexed by the multiplexer switch and itself is a multiplexor
    MultiplexorAndMultiplexedSignal(u64),
    /// Normal signal
    Plain,
}

pub fn parse_multiplexer(text: &str) -> MultiplexIndicator {
    if text == "M" {
        return MultiplexIndicator::Multiplexor;
    }
    if let Some(text) = text.strip_prefix('m') {
        // Multiplexed signal value should be like "m1" or "m1M"
        // Check if it ends with 'M' (multiplexor and multiplexed signal)
        if text.is_empty() {
            // FIXME: is this the right interpretation?
            return MultiplexIndicator::Plain;
        } else if let Some(text) = text.strip_suffix('M') {
            if let Ok(value) = text.parse::<u64>() {
                return MultiplexIndicator::MultiplexorAndMultiplexedSignal(value);
            }
        } else if let Ok(value) = text.parse::<u64>() {
            return MultiplexIndicator::MultiplexedSignal(value);
        }
    }

    panic!("Unknown multiplex indicator: {text}");
}
