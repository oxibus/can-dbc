//! Unit tests for `byte_order`.

use can_dbc::ByteOrder;
use can_dbc_pest::Rule;

use crate::common::test_into;

#[test]
fn byte_order_test() {
    let val = test_into::<ByteOrder>("0", Rule::big_endian);
    assert_eq!(val, ByteOrder::BigEndian);

    let val = test_into::<ByteOrder>("1", Rule::little_endian);
    assert_eq!(val, ByteOrder::LittleEndian);
}
