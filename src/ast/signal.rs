use can_dbc_pest::{Pair, Rule};

use crate::ast::{ByteOrder, MultiplexIndicator, ValueType};
use crate::parser::{
    collect_strings, next, next_optional_rule, next_rule, next_string, parse_min_max_float,
    parse_next_float, parse_next_inner_str, parse_next_uint, validated_inner,
};
use crate::DbcError;

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

/// Parse signal: `SG_ signal_name : start_bit|signal_size@byte_order+/- (factor,offset) [min|max] "unit" receiver`
impl TryFrom<Pair<'_, Rule>> for Signal {
    type Error = DbcError;

    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut pairs = validated_inner(value, Rule::signal)?;

        let name = next_string(&mut pairs, Rule::signal_name)?;
        let multiplexer_indicator =
            if let Some(v) = next_optional_rule(&mut pairs, Rule::multiplexer_indicator) {
                v.as_str().try_into()?
            } else {
                MultiplexIndicator::Plain
            };
        let start_bit = parse_next_uint(&mut pairs, Rule::start_bit)?;
        let size = parse_next_uint(&mut pairs, Rule::signal_size)?;
        let byte_order = next(&mut pairs)?.try_into()?;
        let value_type = next(&mut pairs)?.try_into()?;
        let factor = parse_next_float(&mut pairs, Rule::factor)?;
        let offset = parse_next_float(&mut pairs, Rule::offset)?;
        let (min, max) = parse_min_max_float(next_rule(&mut pairs, Rule::min_max)?)?;
        let unit = parse_next_inner_str(&mut pairs, Rule::unit)?;
        let receivers = collect_strings(&mut pairs, Rule::node_name)?;

        Ok(Self {
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
