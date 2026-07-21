//! Unit tests for `message`.

use can_dbc::{ByteOrder, Message, MessageId, MultiplexIndicator, NumericValue, Signal, ValueType};
use can_dbc_pest::Rule;

use crate::common::test_into;

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
        transmitter: Some("MFA".to_string()),
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
        transmitter: Some("MFA".to_string()),
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

#[test]
fn vector_placeholder_transmitter_test() {
    let def = "BO_ 1 MCA_A1: 6 Vector__XXX";
    let val = test_into::<Message>(def, Rule::message);
    assert_eq!(val.transmitter, None);
}
