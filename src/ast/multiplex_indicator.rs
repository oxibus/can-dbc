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
