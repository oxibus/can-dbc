use crate::ast::{ByteOrder, MultiplexIndicator, ValueType};

/// One or multiple signals are the payload of a CAN frame.
/// To determine the actual value of a signal the following fn applies:
/// `let fnvalue = |can_signal_value| -> can_signal_value * factor + offset;`
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Signal {
    pub name: String,
    pub multiplexer_indicator: MultiplexIndicator,
    pub start_bit: u64,
    pub size: u64,
    pub byte_order: ByteOrder,
    pub value_type: ValueType,
    pub factor: f64,
    pub offset: f64,
    pub min: f64,
    pub max: f64,
    pub unit: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub receivers: Vec<String>,
}
