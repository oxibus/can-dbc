//! Unit tests for `extended_multiplex`.

use can_dbc::{ExtendedMultiplex, ExtendedMultiplexMapping, MessageId};
use can_dbc_pest::Rule;

use crate::common::test_into;

#[test]
fn extended_multiplex_test() {
    // simple mapping
    let def = "
SG_MUL_VAL_ 2147483650 muxed_A_1 MUX_A 1-1;
";
    let exp = ExtendedMultiplex {
        message_id: MessageId::Extended(2),
        signal_name: "muxed_A_1".to_string(),
        multiplexor_signal_name: "MUX_A".to_string(),
        mappings: vec![ExtendedMultiplexMapping {
            min_value: 1,
            max_value: 1,
        }],
    };
    let val = test_into::<ExtendedMultiplex>(def.trim_start(), Rule::sg_mul_val);
    assert_eq!(val, exp);
}

#[test]
fn extended_multiplex_range_test() {
    // range mapping
    let def = "
SG_MUL_VAL_ 2147483650 muxed_A_1 MUX_A 1568-2568;
";
    let exp = ExtendedMultiplex {
        message_id: MessageId::Extended(2),
        signal_name: "muxed_A_1".to_string(),
        multiplexor_signal_name: "MUX_A".to_string(),
        mappings: vec![ExtendedMultiplexMapping {
            min_value: 1568,
            max_value: 2568,
        }],
    };
    let val = test_into::<ExtendedMultiplex>(def.trim_start(), Rule::sg_mul_val);
    assert_eq!(val, exp);
}

#[test]
fn extended_multiplex_mult_test() {
    // multiple mappings
    let def = "
SG_MUL_VAL_ 2147483650 muxed_B_5 MUX_B 5-5, 16-24;
";
    let exp = ExtendedMultiplex {
        message_id: MessageId::Extended(2),
        signal_name: "muxed_B_5".to_string(),
        multiplexor_signal_name: "MUX_B".to_string(),
        mappings: vec![
            ExtendedMultiplexMapping {
                min_value: 5,
                max_value: 5,
            },
            ExtendedMultiplexMapping {
                min_value: 16,
                max_value: 24,
            },
        ],
    };
    let val = test_into::<ExtendedMultiplex>(def.trim_start(), Rule::sg_mul_val);
    assert_eq!(val, exp);
}
