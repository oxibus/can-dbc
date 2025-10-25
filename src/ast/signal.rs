use can_dbc_pest::{Pair, Rule};

use crate::ast::{ByteOrder, MultiplexIndicator, ValueType};
use crate::parser::{parse_float, parse_min_max_float, parse_str, parse_uint, DbcResult};

/// One or multiple signals are the payload of a CAN frame.
/// To determine the actual value of a signal the following fn applies:
/// `let value = |can_signal_value| can_signal_value * factor + offset;`
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

impl Signal {
    /// Parse signal: `SG_ signal_name : start_bit|signal_size@byte_order+/- (factor,offset) [min|max] "unit" receiver`
    pub(crate) fn parse(pair: Pair<Rule>) -> DbcResult<Signal> {
        let mut name = String::new();
        let mut multiplexer_indicator = MultiplexIndicator::Plain;
        let mut start_bit = 0u64;
        let mut size = 0u64;
        let mut byte_order = ByteOrder::BigEndian;
        let mut value_type = ValueType::Unsigned;
        let mut factor = 0.0f64;
        let mut offset = 0.0f64;
        let mut min = 0.0f64;
        let mut max = 0.0f64;
        let mut unit = String::new();
        let mut receivers = Vec::new();

        for pair2 in pair.into_inner() {
            match pair2.as_rule() {
                Rule::signal_name => name = pair2.as_str().to_string(),
                Rule::multiplexer_indicator => {
                    multiplexer_indicator = crate::parse_multiplexer(pair2.as_str());
                }
                Rule::start_bit => start_bit = parse_uint(pair2)?,
                Rule::signal_size => size = parse_uint(pair2)?,
                Rule::big_endian => byte_order = ByteOrder::BigEndian,
                Rule::little_endian => byte_order = ByteOrder::LittleEndian,
                Rule::signed_type => value_type = ValueType::Signed,
                Rule::unsigned_type => value_type = ValueType::Unsigned,
                Rule::factor => factor = parse_float(pair2)?,
                Rule::offset => offset = parse_float(pair2)?,
                Rule::min_max => (min, max) = parse_min_max_float(pair2)?,
                Rule::unit => unit = parse_str(pair2),
                Rule::node_name => receivers.push(pair2.as_str().to_string()),
                _ => panic!("Unexpected rule: {:?}", pair2.as_rule()),
            }
        }

        Ok(Signal {
            name,
            multiplexer_indicator,
            start_bit,
            size,
            byte_order,
            value_type,
            factor,
            offset,
            min,
            max,
            unit,
            receivers,
        })
    }
}
