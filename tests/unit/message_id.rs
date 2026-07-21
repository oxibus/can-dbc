//! Unit tests for `message_id`.

use can_dbc::MessageId;
use can_dbc_pest::Rule;

use crate::common::test_into;

#[test]
#[expect(clippy::unreadable_literal)]
fn extended_message_id_raw() {
    let id = MessageId::Extended(2);
    assert_eq!(id.raw(), 2 | 1 << 31);

    // test with all 29 bits set
    let id = MessageId::Extended(0x1FFF_FFFF);
    assert_eq!(id.raw(), 0b10011111_11111111_11111111_11111111);
}

#[test]
fn standard_message_id_raw() {
    let id = MessageId::Standard(2);
    assert_eq!(id.raw(), 2);
}

#[test]
fn try_from_u32_standard() {
    let id = MessageId::try_from(500u32).unwrap();
    assert_eq!(id, MessageId::Standard(500));
}

#[test]
fn try_from_u32_extended() {
    let id = MessageId::try_from(2u32 | (1 << 31)).unwrap();
    assert_eq!(id, MessageId::Extended(2));
}

#[test]
fn try_from_u64_standard() {
    let id = MessageId::try_from(500u64).unwrap();
    assert_eq!(id, MessageId::Standard(500));
}

#[test]
fn try_from_u64_extended() {
    let id = MessageId::try_from(2u64 | (1 << 31)).unwrap();
    assert_eq!(id, MessageId::Extended(2));
}

#[test]
fn standard_message_id_test() {
    let val = test_into::<MessageId>("2", Rule::message_id);
    assert_eq!(val, MessageId::Standard(2));
}

#[test]
fn extended_low_message_id_test() {
    let s = (2u32 | 1 << 31).to_string();
    let val = test_into::<MessageId>(&s, Rule::message_id);
    assert_eq!(val, MessageId::Extended(2));
}

#[test]
fn extended_message_id_test() {
    let s = (0x1FFF_FFFF_u32 | 1 << 31).to_string();
    let val = test_into::<MessageId>(&s, Rule::message_id);
    assert_eq!(val, MessageId::Extended(0x1FFF_FFFF));
}

#[test]
fn extended_message_id_test_max_29bit() {
    let s = u32::MAX.to_string();
    let val = test_into::<MessageId>(&s, Rule::message_id);
    assert_eq!(val, MessageId::Extended(0x1FFF_FFFF));
}
