//! Unit tests for `signal_extended_value_type_list`.

use can_dbc::{MessageId, SignalExtendedValueType, SignalExtendedValueTypeList};
use can_dbc_pest::Rule;

use crate::common::test_into;

#[test]
fn sig_val_type_test() {
    let def = "
SIG_VALTYPE_ 2000 Signal_8 : 1;
";
    let exp = SignalExtendedValueTypeList {
        message_id: MessageId::Standard(2000),
        signal_name: "Signal_8".to_string(),
        signal_extended_value_type: SignalExtendedValueType::IEEEfloat32Bit,
    };

    let val = test_into::<SignalExtendedValueTypeList>(def.trim_start(), Rule::signal_value_type);
    assert_eq!(val, exp);
}
