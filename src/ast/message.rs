use can_dbc_pest::{Pair, Rule};

use crate::ast::{MessageId, Signal, Transmitter};
use crate::parser::{
    collect_expected, next_rule, next_string, parse_next_uint, single_inner, validated_inner,
};
use crate::DbcError;

/// CAN message (frame) details including signal details
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Message {
    /// CAN id in header of CAN frame.
    /// Must be unique in DBC file.
    pub id: MessageId,
    pub name: String,
    pub size: u64,
    pub transmitter: Transmitter,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub signals: Vec<Signal>,
}

impl TryFrom<Pair<'_, Rule>> for Message {
    type Error = DbcError;

    /// Parse message: `BO_ message_id message_name: message_size transmitter`
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut pairs = validated_inner(value, Rule::message)?;

        // Parse msg_var (contains msg_literal ~ message_id)
        let msg_var_pair = next_rule(&mut pairs, Rule::msg_var)?;
        let id = single_inner(msg_var_pair, Rule::message_id)?.try_into()?;
        let name = next_string(&mut pairs, Rule::message_name)?;
        let size = parse_next_uint(&mut pairs, Rule::message_size)?;

        let transmitter = next_string(&mut pairs, Rule::transmitter)?;
        let transmitter = if matches!(transmitter.as_str(), "Vector__XXX" | "VectorXXX" | "") {
            Transmitter::VectorXXX
        } else {
            Transmitter::NodeName(transmitter)
        };

        let signals = collect_expected(&mut pairs, Rule::signal)?;

        Ok(Self {
            id,
            name,
            size,
            transmitter,
            signals,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ByteOrder, MultiplexIndicator, ValueType};
    use crate::test_helpers::*;
    use crate::NumericValue;

    #[test]
    fn message_definition_test() {
        let def = r#"
BO_ 1 MCA_A1: 6 MFA
 SG_ ABC_1 : 9|2@1+ (1,0) [0|0] "x" XYZ_OUS
 SG_ BasL2 : 3|2@0- (1,0) [0|0] "x" DFA_FUS
"#;

        let exp = Message {
            id: MessageId::Standard(1),
            name: "MCA_A1".to_string(),
            size: 6,
            transmitter: Transmitter::NodeName("MFA".to_string()),
            signals: vec![
                Signal {
                    name: "ABC_1".to_string(),
                    start_bit: 9,
                    size: 2,
                    byte_order: ByteOrder::LittleEndian,
                    value_type: ValueType::Unsigned,
                    factor: 1.0,
                    offset: 0.0,
                    min: NumericValue::Uint(0),
                    max: NumericValue::Uint(0),
                    unit: "x".to_string(),
                    multiplexer_indicator: MultiplexIndicator::Plain,
                    receivers: vec!["XYZ_OUS".to_string()],
                },
                Signal {
                    name: "BasL2".to_string(),
                    start_bit: 3,
                    size: 2,
                    byte_order: ByteOrder::BigEndian,
                    value_type: ValueType::Signed,
                    factor: 1.0,
                    offset: 0.0,
                    min: NumericValue::Uint(0),
                    max: NumericValue::Uint(0),
                    unit: "x".to_string(),
                    multiplexer_indicator: MultiplexIndicator::Plain,
                    receivers: vec!["DFA_FUS".to_string()],
                },
            ],
        };
        let val = test_into::<Message>(def.trim_start(), Rule::message);
        assert_eq!(val, exp);
    }

    #[test]
    fn min_max_numeric_test() {
        let def = r#"BO_ 1 MCA_A1: 6 MFA
 SG_ uint : 9|2@1+ (1,0) [0|18446744073709551615] "x" XYZ_OUS
 SG_ int : 3|2@0- (1,0) [-9223372036854775808|9223372036854775807] "x" DFA_FUS
"#;

        let exp = Message {
            id: MessageId::Standard(1),
            name: "MCA_A1".to_string(),
            size: 6,
            transmitter: Transmitter::NodeName("MFA".to_string()),
            signals: vec![
                Signal {
                    name: "uint".to_string(),
                    start_bit: 9,
                    size: 2,
                    byte_order: ByteOrder::LittleEndian,
                    value_type: ValueType::Unsigned,
                    factor: 1.0,
                    offset: 0.0,
                    min: NumericValue::Uint(0),
                    max: NumericValue::Uint(18_446_744_073_709_551_615),
                    unit: "x".to_string(),
                    multiplexer_indicator: MultiplexIndicator::Plain,
                    receivers: vec!["XYZ_OUS".to_string()],
                },
                Signal {
                    name: "int".to_string(),
                    start_bit: 3,
                    size: 2,
                    byte_order: ByteOrder::BigEndian,
                    value_type: ValueType::Signed,
                    factor: 1.0,
                    offset: 0.0,
                    min: NumericValue::Int(-9_223_372_036_854_775_808),
                    max: NumericValue::Uint(9_223_372_036_854_775_807),
                    unit: "x".to_string(),
                    multiplexer_indicator: MultiplexIndicator::Plain,
                    receivers: vec!["DFA_FUS".to_string()],
                },
            ],
        };
        let val = test_into::<Message>(def.trim_start(), Rule::message);
        assert_eq!(val, exp);
    }
}
