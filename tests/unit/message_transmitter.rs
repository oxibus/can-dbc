//! Unit tests for `message_transmitter`.

use can_dbc::{MessageId, MessageTransmitter};
use can_dbc_pest::Rule;

use crate::common::test_into;

#[test]
fn message_transmitters_test() {
    let def = "
BO_TX_BU_ 12345 : XZY,ABC;
";
    let exp = MessageTransmitter {
        message_id: MessageId::Standard(12345),
        transmitter: vec!["XZY".to_string(), "ABC".to_string()],
    };
    let val = test_into::<MessageTransmitter>(def.trim_start(), Rule::message_transmitter);
    assert_eq!(val, exp);

    // Same as above, but without space before the colon
    let def = "
BO_TX_BU_ 12345 :XZY,ABC;
";
    let val = test_into::<MessageTransmitter>(def.trim_start(), Rule::message_transmitter);
    assert_eq!(val, exp);

    let def = "
BO_TX_BU_ 12345 : Vector__XXX;
";
    let val = test_into::<MessageTransmitter>(def.trim_start(), Rule::message_transmitter);
    assert_eq!(val.transmitter, Vec::<String>::new());
}
