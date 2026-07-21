//! Unit tests for `multiplex_indicator`.

use can_dbc::MultiplexIndicator;

#[test]
fn multiplexer_indicator_test() {
    let val: MultiplexIndicator = "m34920".try_into().unwrap();
    assert_eq!(val, MultiplexIndicator::MultiplexedSignal(34920));

    let val: MultiplexIndicator = "M".try_into().unwrap();
    assert_eq!(val, MultiplexIndicator::Multiplexor);

    // Empty string is not a valid multiplexer indicator, so we skip this test
    // let val: MultiplexIndicator = "".try_into().unwrap();
    // assert_eq!(val, MultiplexIndicator::Plain);

    let val: MultiplexIndicator = "m8M".try_into().unwrap();
    assert_eq!(val, MultiplexIndicator::MultiplexorAndMultiplexedSignal(8));
}
