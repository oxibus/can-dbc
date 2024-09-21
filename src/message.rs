#[cfg(feature = "with-serde")]
extern crate serde;
#[cfg(feature = "with-serde")]
#[macro_use]
extern crate serde_derive;

use derive_getters::Getters;

use crate::parser;
use crate::DBCObject;
use crate::Signal;
use crate::Transmitter;

use nom::{
    bytes::complete::tag,
    character::complete::{self, multispace0},
    multi::many0,
};

/// CAN id in header of CAN frame.
/// Must be unique in DBC file.
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum MessageId {
    Standard(u16),
    /// 29 bit extended identifier without the extended bit.
    /// For the raw value of the message id including the bit for extended identifiers use the `raw()` method.
    Extended(u32),
}

impl MessageId {
    /// Raw value of the message id including the bit for extended identifiers
    pub fn raw(&self) -> u32 {
        match self {
            MessageId::Standard(id) => *id as u32,
            MessageId::Extended(id) => *id | 1 << 31,
        }
    }
}

impl DBCObject for MessageId {
    fn dbc_string(&self) -> String {
        return match self {
            Self::Standard(id) => (*id as u32).to_string(),
            Self::Extended(id) => (*id as u32 + (1 << 31)).to_string(),
        };
    }

    fn parse(s: &str) -> nom::IResult<&str, Self>
    where
        Self: Sized,
    {
        let (s, parsed_value) = complete::u32(s)?;

        if parsed_value & (1 << 31) != 0 {
            Ok((s, MessageId::Extended(parsed_value & 0x1FFFFFFF)))
        } else {
            Ok((s, MessageId::Standard(parsed_value as u16)))
        }
    }
}

#[test]
fn standard_message_id_test() {
    let s = "2";
    let (_, extended_message_id) = MessageId::parse(s).unwrap();

    // Test parsing
    assert_eq!(extended_message_id, MessageId::Standard(2));

    // Test generation
    assert_eq!(s, extended_message_id.dbc_string());
}

#[test]
fn extended_low_message_id_test() {
    let s = (2u32 | 1 << 31).to_string();
    let (_, extended_message_id) = MessageId::parse(&s).unwrap();

    // Test parsing
    assert_eq!(extended_message_id, MessageId::Extended(2));

    // Test generation
    assert_eq!(s, extended_message_id.dbc_string());
}

#[test]
fn extended_message_id_test() {
    let s = (0x1FFFFFFFu32 | 1 << 31).to_string();
    let (_, extended_message_id) = MessageId::parse(&s).unwrap();

    // Test parsing
    assert_eq!(extended_message_id, MessageId::Extended(0x1FFFFFFF));

    // Test generation
    assert_eq!(s, extended_message_id.dbc_string());
}

#[test]
fn extended_message_id_test2() {
    let id = 2684354559;
    let s = "2684354559";
    let (_, extended_message_id) = MessageId::parse(&s).unwrap();

    // Test parsing
    assert_eq!(extended_message_id, MessageId::Extended(id & 0x1FFFFFFFu32));

    // Test generation
    assert_eq!(s, extended_message_id.dbc_string());

    let id = 2204433193;
    let s = "2204433193";
    let (_, extended_message_id) = MessageId::parse(&s).unwrap();

    // Test parsing
    assert_eq!(extended_message_id, MessageId::Extended(id & 0x1FFFFFFFu32));

    // Test generation
    assert_eq!(s, extended_message_id.dbc_string());
}

#[test]
fn extended_message_id_test_max_29bit() {
    let s = u32::MAX.to_string();
    let (_, extended_message_id) = MessageId::parse(&s).unwrap();

    // Test parsing
    assert_eq!(extended_message_id, MessageId::Extended(0x1FFFFFFF));

    // Test generation
    assert_eq!(0x9FFFFFFFu32.to_string(), extended_message_id.dbc_string());
    // This is not the same value we passed in as the maximum value of an extended ID is
    // 0x1FFFFFFF and it will also return the leftmost bit set high to indicate that it's
    // an extended ID.
}

/// CAN message (frame) details including signal details
#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Message {
    /// CAN id in header of CAN frame.
    /// Must be unique in DBC file.
    pub(crate) message_id: MessageId,
    pub(crate) message_name: String,
    pub(crate) message_size: u64,
    pub(crate) transmitter: Transmitter,
    pub(crate) signals: Vec<Signal>,
}

impl DBCObject for Message {
    fn dbc_string(&self) -> String {
        return format!(
            "BO_ {} {}: {} {}\n {}",
            self.message_id.dbc_string(),
            self.message_name,
            self.message_size,
            self.transmitter.dbc_string(),
            self.signals
                .clone()
                .into_iter()
                .map(|sg| sg.dbc_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }

    fn parse(s: &str) -> nom::IResult<&str, Self>
    where
        Self: Sized,
    {
        let (s, _) = multispace0(s)?;
        let (s, _) = tag("BO_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, message_id) = MessageId::parse(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, message_name) = parser::c_ident(s)?;
        let (s, _) = parser::colon(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, message_size) = complete::u64(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, transmitter) = Transmitter::parse(s)?;
        let (s, signals) = many0(Signal::parse)(s)?;
        Ok((
            s,
            (Message {
                message_id,
                message_name,
                message_size,
                transmitter,
                signals,
            }),
        ))
    }
}

#[test]
fn message_definition_test() {
    let def = "BO_ 1 MCA_A1: 6 MFA\r\nSG_ ABC_1 : 9|2@1+ (1,0) [0|0] \"x\" XYZ_OUS\r\nSG_ BasL2 : 3|2@0- (1,0) [0|0] \"x\" DFA_FUS\r\n x";
    Signal::parse("\r\n\r\nSG_ BasL2 : 3|2@0- (1,0) [0|0] \"x\" DFA_FUS\r\n").expect("Failed");
    let (_, _message_def) = Message::parse(def).expect("Failed to parse message definition");

    // todo!("test a correct definition");
}
