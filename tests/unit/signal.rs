//! Unit tests for `signal`.

use can_dbc::{ByteOrder, MultiplexIndicator, NumericValue, Signal, ValueType};
use can_dbc_pest::Rule;

use crate::common::test_into;

#[test]
fn signal_test() {
    let def = r#"
 SG_ NAME : 3|2@1- (1,0) [0|0] "x" UFA
"#;

    let exp = Signal {
        name: "NAME".to_string(),
        start_bit: 3,
        size: 2,
        byte_order: ByteOrder::LittleEndian,
        value_type: ValueType::Signed,
        factor: 1.0,
        offset: 0.0,
        min: NumericValue::Uint(0),
        max: NumericValue::Uint(0),
        unit: "x".to_string(),
        multiplexer_indicator: MultiplexIndicator::Plain,
        receivers: vec!["UFA".to_string()],
    };
    let val = test_into::<Signal>(def, Rule::signal);
    assert_eq!(val, exp);
}

#[test]
fn signal_definition_test() {
    // multiple newlines with optional spaces/comments before each signal line
    let def = "\r\n \r\n SG_ BasL2 : 3|2@0- (1,0) [0|0] \"x\" DFA_FUS\r\n";

    let exp = Signal {
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
    };
    let val = test_into::<Signal>(def, Rule::signal);
    assert_eq!(val, exp);
}

#[test]
fn vector_placeholder_receiver_test() {
    let def = r#"
 SG_ NAME : 3|2@1- (1,0) [0|0] "x" Vector__XXX
"#;
    let val = test_into::<Signal>(def, Rule::signal);
    assert_eq!(val.receivers, Vec::<String>::new());
}
