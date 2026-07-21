//! Unit tests for `signal_groups`.

use can_dbc::{MessageId, SignalGroups};
use can_dbc_pest::Rule;

use crate::common::test_into;

#[test]
fn signal_groups_test() {
    let def = "
SIG_GROUP_ 23 X_3290 1 : A_b XY_Z;
";
    let exp = SignalGroups {
        message_id: MessageId::Standard(23),
        name: "X_3290".to_string(),
        repetitions: 1,
        signal_names: vec!["A_b".to_string(), "XY_Z".to_string()],
    };
    let val = test_into::<SignalGroups>(def.trim_start(), Rule::signal_group);
    assert_eq!(val, exp);
}
