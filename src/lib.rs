//!
//! A CAN database (dbc) format parser written with Rust's nom parser combinator library.
//! CAN databases are used to exchange details about a CAN network.
//! E.g. what messages are being send over the CAN bus and what data do they contain.
//!
//! ```rust
//! use can_dbc::DBC;
//! use codegen::Scope;
//!
//! use std::fs::File;
//! use std::io;
//! use std::io::prelude::*;
//!
//! fn main() -> io::Result<()> {
//!     let mut f = File::open("./examples/sample.dbc")?;
//!     let mut buffer = Vec::new();
//!     f.read_to_end(&mut buffer)?;
//!
//!     let dbc = can_dbc::DBC::from_slice(&buffer).expect("Failed to parse dbc file");
//!
//!     let mut scope = Scope::new();
//!     for message in dbc.messages() {
//!         for signal in message.signals() {
//!
//!             let mut scope = Scope::new();
//!             let message_struct = scope.new_struct(message.message_name());
//!             for signal in message.signals() {
//!                 message_struct.field(signal.name().to_lowercase().as_str(), "f64");
//!             }
//!         }
//!     }
//!
//!     println!("{}", scope.to_string());
//!     Ok(())
//! }
//! ```

#[cfg(feature = "with-serde")]
extern crate serde;
#[cfg(feature = "with-serde")]
#[macro_use]
extern crate serde_derive;

use std::convert::TryFrom;

use derive_getters::Getters;

pub mod parser;

trait DBCString {
    fn dbc_string(&self) -> String;
}

#[cfg(test)]
mod tests {

    use super::*;

    const SAMPLE_DBC: &str = r#"
VERSION "0.1"
NS_ :
    NS_DESC_
    CM_
    BA_DEF_
    BA_
    VAL_
    CAT_DEF_
    CAT_
    FILTER
    BA_DEF_DEF_
    EV_DATA_
    ENVVAR_DATA_
    SGTYPE_
    SGTYPE_VAL_
    BA_DEF_SGTYPE_
    BA_SGTYPE_
    SIG_TYPE_REF_
    VAL_TABLE_
    SIG_GROUP_
    SIG_VALTYPE_
    SIGTYPE_VALTYPE_
    BO_TX_BU_
    BA_DEF_REL_
    BA_REL_
    BA_DEF_DEF_REL_
    BU_SG_REL_
    BU_EV_REL_
    BU_BO_REL_
    SG_MUL_VAL_
BS_:
BU_: PC
BO_ 2000 WebData_2000: 4 Vector__XXX
    SG_ Signal_8 : 24|8@1+ (1,0) [0|255] "" Vector__XXX
    SG_ Signal_7 : 16|8@1+ (1,0) [0|255] "" Vector__XXX
    SG_ Signal_6 : 8|8@1+ (1,0) [0|255] "" Vector__XXX
    SG_ Signal_5 : 0|8@1+ (1,0) [0|255] "" Vector__XXX
BO_ 1840 WebData_1840: 4 PC
    SG_ Signal_4 : 24|8@1+ (1,0) [0|255] "" Vector__XXX
    SG_ Signal_3 : 16|8@1+ (1,0) [0|255] "" Vector__XXX
    SG_ Signal_2 : 8|8@1+ (1,0) [0|255] "" Vector__XXX
    SG_ Signal_1 : 0|8@1+ (1,0) [0|0] "" Vector__XXX

BO_ 3040 WebData_3040: 8 Vector__XXX
    SG_ Signal_6 m2 : 0|4@1+ (1,0) [0|15] "" Vector__XXX
    SG_ Signal_5 m3 : 16|8@1+ (1,0) [0|255] "kmh" Vector__XXX
    SG_ Signal_4 m3 : 8|8@1+ (1,0) [0|255] "" Vector__XXX
    SG_ Signal_3 m3 : 0|4@1+ (1,0) [0|3] "" Vector__XXX
    SG_ Signal_2 m1 : 3|12@0+ (1,0) [0|4095] "Byte" Vector__XXX
    SG_ Signal_1 m0 : 0|4@1+ (1,0) [0|7] "Byte" Vector__XXX
    SG_ Switch M : 4|4@1+ (1,0) [0|3] "" Vector__XXX

EV_ Environment1: 0 [0|220] "" 0 6 DUMMY_NODE_VECTOR0 DUMMY_NODE_VECTOR2;
EV_ Environment2: 0 [0|177] "" 0 7 DUMMY_NODE_VECTOR1 DUMMY_NODE_VECTOR2;
ENVVAR_DATA_ SomeEnvVarData: 399;

CM_ BO_ 1840 "Some Message comment";
CM_ SG_ 1840 Signal_4 "asaklfjlsdfjlsdfgls
HH?=(%)/&KKDKFSDKFKDFKSDFKSDFNKCnvsdcvsvxkcv";
CM_ SG_ 5 TestSigLittleUnsigned1 "asaklfjlsdfjlsdfgls
=0943503450KFSDKFKDFKSDFKSDFNKCnvsdcvsvxkcv";

BA_DEF_DEF_ "BusType" "AS";

BA_ "Attr" BO_ 4358435 283;
BA_ "Attr" BO_ 56949545 344;

VAL_ 2000 Signal_3 255 "NOP";

SIG_VALTYPE_ 2000 Signal_8 : 1;
"#;

    #[test]
    fn dbc_definition_test() {
        match DBC::try_from(SAMPLE_DBC) {
            Ok(dbc_content) => println!("DBC Content{:#?}", dbc_content),
            Err(e) => {
                match e {
                    Error::Nom(nom::Err::Incomplete(needed)) => {
                        eprintln!("Error incomplete input, needed: {:?}", needed)
                    }
                    Error::Nom(nom::Err::Error(error)) => {
                        eprintln!("Nom Error: {:?}", error);
                    }
                    Error::Nom(nom::Err::Failure(ctx)) => eprintln!("Failure {:?}", ctx),
                    Error::Incomplete(dbc, remaining) => eprintln!(
                        "Not all data in buffer was read {:#?}, remaining unparsed: {}",
                        dbc, remaining
                    ),
                    Error::MultipleMultiplexors => eprintln!("Multiple multiplexors defined"),
                }
                panic!("Failed to read DBC");
            }
        }
    }

    #[test]
    fn lookup_signal_comment() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        let comment = dbc_content
            .signal_comment(MessageId::Standard(1840), "Signal_4")
            .expect("Signal comment missing");
        assert_eq!(
            "asaklfjlsdfjlsdfgls\nHH?=(%)/&KKDKFSDKFKDFKSDFKSDFNKCnvsdcvsvxkcv",
            comment
        );
    }

    #[test]
    fn lookup_signal_comment_none_when_missing() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        let comment = dbc_content.signal_comment(MessageId::Standard(1840), "Signal_2");
        assert_eq!(None, comment);
    }

    #[test]
    fn lookup_message_comment() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        let comment = dbc_content
            .message_comment(MessageId::Standard(1840))
            .expect("Message comment missing");
        assert_eq!("Some Message comment", comment);
    }

    #[test]
    fn lookup_message_comment_none_when_missing() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        let comment = dbc_content.message_comment(MessageId::Standard(2000));
        assert_eq!(None, comment);
    }

    #[test]
    fn lookup_value_descriptions_for_signal() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        let val_descriptions = dbc_content
            .value_descriptions_for_signal(MessageId::Standard(2000), "Signal_3")
            .expect("Message comment missing");

        let exp = vec![ValDescription {
            a: 255.0,
            b: "NOP".to_string(),
        }];
        assert_eq!(exp, val_descriptions);
    }

    #[test]
    fn lookup_value_descriptions_for_signal_none_when_missing() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        let val_descriptions =
            dbc_content.value_descriptions_for_signal(MessageId::Standard(2000), "Signal_2");
        assert_eq!(None, val_descriptions);
    }

    #[test]
    fn lookup_extended_value_type_for_signal() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        let extended_value_type =
            dbc_content.extended_value_type_for_signal(MessageId::Standard(2000), "Signal_8");
        assert_eq!(
            extended_value_type,
            Some(&SignalExtendedValueType::IEEEfloat32Bit)
        );
    }

    #[test]
    fn lookup_extended_value_type_for_signal_none_when_missing() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        let extended_value_type =
            dbc_content.extended_value_type_for_signal(MessageId::Standard(2000), "Signal_1");
        assert_eq!(extended_value_type, None);
    }

    #[test]
    fn lookup_signal_by_name() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        let signal = dbc_content.signal_by_name(MessageId::Standard(2000), "Signal_8");
        assert!(signal.is_some());
    }

    #[test]
    fn lookup_signal_by_name_none_when_missing() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        let signal = dbc_content.signal_by_name(MessageId::Standard(2000), "Signal_25");
        assert_eq!(signal, None);
    }

    #[test]
    fn lookup_multiplex_indicator_switch() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        let multiplexor_switch = dbc_content.message_multiplexor_switch(MessageId::Standard(3040));
        assert!(multiplexor_switch.is_ok());
        assert!(multiplexor_switch.as_ref().unwrap().is_some());
        assert_eq!(multiplexor_switch.unwrap().unwrap().name(), "Switch");
    }

    #[test]
    fn lookup_multiplex_indicator_switch_none_when_missing() {
        let dbc_content = DBC::try_from(SAMPLE_DBC).expect("Failed to parse DBC");
        let multiplexor_switch = dbc_content.message_multiplexor_switch(MessageId::Standard(1840));
        assert!(multiplexor_switch.unwrap().is_none());
    }

    #[test]
    fn extended_message_id_raw() {
        let message_id = MessageId::Extended(2);
        assert_eq!(message_id.raw(), 2 | 1 << 31);
        let message_id = MessageId::Extended(2 ^ 29);
        assert_eq!(message_id.raw(), 2 ^ 29 | 1 << 31);
    }

    #[test]
    fn standard_message_id_raw() {
        let message_id = MessageId::Standard(2);
        assert_eq!(message_id.raw(), 2);
    }
}

/// Possible error cases for `can-dbc`
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Error<'a> {
    /// Remaining String, the DBC was only read partially.
    /// Occurs when e.g. an unexpected symbol occurs.
    Incomplete(DBC, &'a str),
    /// Parser failed
    Nom(nom::Err<nom::error::Error<&'a str>>),
    /// Can't Lookup multiplexors because the message uses extended multiplexing.
    MultipleMultiplexors,
}

/// Baudrate of network in kbit/s
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Baudrate(u64);

impl DBCString for Baudrate {
    fn dbc_string(&self) -> String {
        return self.0.to_string()
    }
}

/// One or multiple signals are the payload of a CAN frame.
/// To determine the actual value of a signal the following fn applies:
/// `let fnvalue = |can_signal_value| -> can_signal_value * factor + offset;`
#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Signal {
    name: String,
    multiplexer_indicator: MultiplexIndicator,
    pub start_bit: u64,
    pub signal_size: u64,
    byte_order: ByteOrder,
    value_type: ValueType,
    pub factor: f64,
    pub offset: f64,
    pub min: f64,
    pub max: f64,
    unit: String,
    receivers: Vec<String>,
}

impl DBCString for Signal {
    fn dbc_string(&self) -> String {
        let receivers = match self.receivers.len() {
            0 => "Vector__XXX".to_string(),
            _ => self.receivers.join(", "),
        };
        // format! macro doesn't support direct field access inline with the string
        return format!(r##"SG {} {}: {}|{}@{}{} ({},{}) [{}|{}] "{}" {}"##,
            self.name,
            self.multiplexer_indicator.dbc_string(), // TODO handle the trailing space?
            self.start_bit,
            self.signal_size,
            self.byte_order.dbc_string(),
            self.value_type.dbc_string(),
            self.factor,
            self.offset,
            self.min,
            self.max,
            self.unit,
            receivers
        )
    }
}

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

impl DBCString for MessageId {
    fn dbc_string(&self) -> String {
        return match self {
            Self::Standard(id) => id.to_string(),
            Self::Extended(id) => id.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Transmitter {
    /// node transmitting the message
    NodeName(String),
    /// message has no sender
    VectorXXX,
}

impl DBCString for Transmitter {
    fn dbc_string(&self) -> String {
        return match self {
            Self::NodeName(s) => s.to_string(),
            Self::VectorXXX => "Vector__XXX".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MessageTransmitter {
    message_id: MessageId,
    transmitter: Vec<Transmitter>,
}

impl DBCString for MessageTransmitter {
    fn dbc_string(&self) -> String {
        return format!("BO_TX_BU_ {} : {}",
          self.message_id.dbc_string(),
          self.transmitter
            .clone()
            .into_iter()
            .map(|t| t.dbc_string())
            .collect::<Vec<String>>()
            .join(","),
            // TODO determine if it will be a problem to kick out Vector__XXX if no transmitter is defined
        )
    }
}

/// Version generated by DB editor
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Version(pub String);

impl DBCString for Version {
    fn dbc_string(&self) -> String {
        return format!("VERSION {}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Symbol(pub String);

impl DBCString for Symbol {
    fn dbc_string(&self) -> String {
        return self.0.to_string()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum MultiplexIndicator {
    /// Multiplexor switch
    Multiplexor,
    /// Signal us being multiplexed by the multiplexer switch.
    MultiplexedSignal(u64),
    /// Signal us being multiplexed by the multiplexer switch and itself is a multiplexor
    MultiplexorAndMultiplexedSignal(u64),
    /// Normal signal
    Plain,
}

impl DBCString for MultiplexIndicator {
    fn dbc_string(&self) -> String {
        return match self {
            Self::Multiplexor => "M".to_string(),
            Self::MultiplexedSignal(m) => format!("m{m}").to_string(),
            Self::MultiplexorAndMultiplexedSignal(m) => format!("M{m}").to_string(),
            Self::Plain => "".to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}

impl DBCString for ByteOrder {
    fn dbc_string(&self) -> String {
        return match self {
            Self::LittleEndian => "1".to_string(),
            Self::BigEndian => "0".to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum ValueType {
    Signed,
    Unsigned,
}

impl DBCString for ValueType {
    fn dbc_string(&self) -> String {
        return match self {
            Self::Signed => "-".to_string(),
            Self::Unsigned => "+".to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum EnvType {
    EnvTypeFloat,
    EnvTypeu64,
    EnvTypeData,
}

impl DBCString for EnvType {
    fn dbc_string(&self) -> String {
        return match self {
            Self::EnvTypeFloat => "0".to_string(),
            Self::EnvTypeu64 => "1".to_string(),
            Self::EnvTypeData => "".to_string(), // TODO determine what this value should enumerate to
        }
    }
}

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SignalType {
    signal_type_name: String,
    signal_size: u64,
    byte_order: ByteOrder,
    value_type: ValueType,
    factor: f64,
    offset: f64,
    min: f64,
    max: f64,
    unit: String,
    default_value: f64,
    value_table: String,
}

impl DBCString for SignalType {
    fn dbc_string(&self) -> String {
        // TODO this is difficult to test since CANdb++ doesn't seem to have this feature implemented
        return format!("SGTYPE_ {}: {}@{}{} ({},{}) [{}|{}] {} {} {};",
          self.signal_type_name,
          self.signal_size,
          self.byte_order.dbc_string(),
          self.value_type.dbc_string(),
          self.factor,
          self.offset,
          self.min,
          self.max,
          self.unit, // TODO figure out if I need to escape the unit quotes with backslashes
          self.default_value,
          self.value_table,
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AccessType {
    DummyNodeVector0,
    DummyNodeVector1,
    DummyNodeVector2,
    DummyNodeVector3,
}

impl DBCString for AccessType {
    fn dbc_string(&self) -> String {
        return format!("DUMMY_NODE_VECTOR{}",
          match self {
            Self::DummyNodeVector0 => "0",
            Self::DummyNodeVector1 => "1",
            Self::DummyNodeVector2 => "2",
            Self::DummyNodeVector3 => "3",
          }
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AccessNode {
    AccessNodeVectorXXX,
    AccessNodeName(String),
}

impl DBCString for AccessNode {
    fn dbc_string(&self) -> String {
        return match self {
            Self::AccessNodeName(s) => s.to_string(),
            Self::AccessNodeVectorXXX => "Vector__XXX".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum SignalAttributeValue {
    Text(String),
    Int(i64),
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AttributeValuedForObjectType {
    RawAttributeValue(AttributeValue),
    NetworkNodeAttributeValue(String, AttributeValue),
    MessageDefinitionAttributeValue(MessageId, Option<AttributeValue>),
    SignalAttributeValue(MessageId, String, AttributeValue),
    EnvVariableAttributeValue(String, AttributeValue),
}

impl DBCString for AttributeValuedForObjectType {
    fn dbc_string(&self) -> String {
        return match self {
         Self::RawAttributeValue(av) => av.dbc_string(),
         Self::NetworkNodeAttributeValue(node_name, av) => {
            format!("BU_ {} {} ", node_name, av.dbc_string())
         },
         Self::MessageDefinitionAttributeValue(m_id, av) => {
            format!("BO_ {}{}",
                m_id.dbc_string(),
                match av {
                    None => "".to_string(),
                    Some(v) => format!(" {}", v.dbc_string()),
                }
            )
         },
         Self::SignalAttributeValue(m_id, s, av) => {
            format!("SG_ {} {} {}", m_id.dbc_string(), s, av.dbc_string())
         },
         Self::EnvVariableAttributeValue(s, av) => {
            format!("EV_ {} {}", s, av.dbc_string())
         },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AttributeValueType {
    AttributeValueTypeInt(i64, i64),
    AttributeValueTypeHex(i64, i64),
    AttributeValueTypeFloat(f64, f64),
    AttributeValueTypeString,
    AttributeValueTypeEnum(Vec<String>),
}

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ValDescription {
    a: f64,
    b: String,
}

impl DBCString for ValDescription {
    fn dbc_string(&self) -> String {
        return format!("{} \"{}\"",
            self.a, self.b
        )
    }
}

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct AttrDefault {
    name: String,
    value: AttributeValue,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum AttributeValue {
    AttributeValueU64(u64),
    AttributeValueI64(i64),
    AttributeValueF64(f64),
    AttributeValueCharString(String),
}

impl DBCString for AttributeValue {
    fn dbc_string(&self) -> String {
        return match self {
            Self::AttributeValueU64(val) => val.to_string(),
            Self::AttributeValueI64(val) => val.to_string(),
            Self::AttributeValueF64(val) => val.to_string(),
            Self::AttributeValueCharString(val) => val.to_string(),
        }
    }
}

/// Global value table
#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ValueTable {
    value_table_name: String,
    value_descriptions: Vec<ValDescription>,
}

impl DBCString for ValueTable {
    fn dbc_string(&self) -> String {
        return format!("VAL_TABLE_ {} {}",
            self.value_table_name,
            self.value_descriptions
                .clone()
                .into_iter()
                .map(|vd| vd.dbc_string())
                .collect::<Vec<String>>()
                .join(";")
        )
    }
}

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ExtendedMultiplexMapping {
    min_value: u64,
    max_value: u64,
}

impl DBCString for ExtendedMultiplexMapping {
    fn dbc_string(&self) -> String {
        return format!("{}-{}", self.min_value, self.max_value)
    }
}

/// Mapping between multiplexors and multiplexed signals
#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ExtendedMultiplex {
    message_id: MessageId,
    signal_name: String,
    multiplexor_signal_name: String,
    mappings: Vec<ExtendedMultiplexMapping>,
}

impl DBCString for ExtendedMultiplex {
    fn dbc_string(&self) -> String {
        return format!("SG_MUL_VAL_ {} {} {} {}",
            self.message_id.dbc_string(),
            self.signal_name,
            self.multiplexor_signal_name,
            self.mappings
                .clone()
                .into_iter()
                .map(|m| m.dbc_string())
                .collect::<Vec<String>>()
                .join(";")
        )
    }
}

/// Object comments
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Comment {
    Node {
        node_name: String,
        comment: String,
    },
    Message {
        message_id: MessageId,
        comment: String,
    },
    Signal {
        message_id: MessageId,
        signal_name: String,
        comment: String,
    },
    EnvVar {
        env_var_name: String,
        comment: String,
    },
    Plain {
        comment: String,
    },
}

impl DBCString for Comment {
    fn dbc_string(&self) -> String {
        return match self {
            Self::Node{node_name, comment} => {
                format!("BU_ {} \"{}\"",
                    node_name,
                    comment
                )
            },
            Self::Message{message_id, comment} => {
                format!("BO_ {} \"{}\"",
                    message_id.dbc_string(),
                    comment,
                )
            },
            Self::Signal{message_id, signal_name, comment} => {
                format!("SG_ {} {} \"{}\"",
                    message_id.dbc_string(),
                    signal_name,
                    comment
                )
            },
            Self::EnvVar{env_var_name, comment} => {
                format!("EV_ {} \"{}\"",
                    env_var_name,
                    comment,
                )
            },
            Self::Plain{comment} => format!("\"{}\"", comment),
        }
    }
}


/// CAN message (frame) details including signal details
#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Message {
    /// CAN id in header of CAN frame.
    /// Must be unique in DBC file.
    message_id: MessageId,
    message_name: String,
    message_size: u64,
    transmitter: Transmitter,
    signals: Vec<Signal>,
}

impl DBCString for Message {
    fn dbc_string(&self) -> String {
        return format!("BO_ {} {}: {} {}\n  {}",
            self.message_id.dbc_string(),
            self.message_name,
            self.message_size,
            self.transmitter.dbc_string(),
            self.signals
                .clone()
                .into_iter()
                .map(|sg| sg.dbc_string())
                .collect::<Vec<String>>()
                .join("\n  ")
        )
    }
}

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct EnvironmentVariable {
    env_var_name: String,
    env_var_type: EnvType,
    min: i64,
    max: i64,
    unit: String,
    initial_value: f64,
    ev_id: i64,
    access_type: AccessType,
    access_nodes: Vec<AccessNode>,
}

impl DBCString for EnvironmentVariable {
    fn dbc_string(&self) -> String {
        return format!("EV_ {}: {} [{}|{}] \"{}\" {} {} {} {};",
            self.env_var_name,
            self.env_var_type.dbc_string(),
            self.min,
            self.max,
            self.unit,
            self.initial_value,
            self.ev_id,
            self.access_type.dbc_string(),
            self.access_nodes
                .clone()
                .into_iter()
                .map(|an| an.dbc_string())
                .collect::<Vec<String>>()
                .join(";")
        )
    }
}

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct EnvironmentVariableData {
    env_var_name: String,
    data_size: u64,
}

impl DBCString for EnvironmentVariableData {
    fn dbc_string(&self) -> String {
        return format!("ENVVAR_DATA_ {}: {};",
            self.env_var_name,
            self.data_size,
        )
    }
}

/// CAN network nodes, names must be unique
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Node(pub Vec<String>);

impl DBCString for Node {
    fn dbc_string(&self) -> String {
        return format!("BU_: {}",
            self.0.clone().join(" ")
        )
    }
}

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct AttributeDefault {
    attribute_name: String,
    attribute_value: AttributeValue,
}

impl DBCString for AttributeDefault {
    fn dbc_string(&self) -> String {
        return format!("BA_DEF_DEF_ \"{}\" {};",
            self.attribute_name,
            self.attribute_value.dbc_string(),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct AttributeValueForObject {
    attribute_name: String,
    attribute_value: AttributeValuedForObjectType,
}

impl DBCString for AttributeValueForObject {
    fn dbc_string(&self) -> String {
        return format!("BA_ \"{}\" {};",
            self.attribute_name,
            self.attribute_value.dbc_string(),
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AttributeDefinition {
    // TODO add properties
    Message(String),
    // TODO add properties
    Node(String),
    // TODO add properties
    Signal(String),
    EnvironmentVariable(String),
    // TODO figure out name
    Plain(String),
}

impl DBCString for AttributeDefinition {
    fn dbc_string(&self) -> String {
        return match self {
            Self::Message(msg) => format!("BO_ {};", msg),
            Self::Node(node) => format!("BU_ {};", node),
            Self::Signal(sig) => format!("SG_ {};", sig),
            Self::EnvironmentVariable(ev) => format!("EV_ {};", ev),
            Self:: Plain(s) => format!("{};", s),
        }
    }
}

/// Encoding for signal raw values.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum ValueDescription {
    Signal {
        message_id: MessageId,
        signal_name: String,
        value_descriptions: Vec<ValDescription>,
    },
    EnvironmentVariable {
        env_var_name: String,
        value_descriptions: Vec<ValDescription>,
    },
}

impl DBCString for ValueDescription {
    fn dbc_string(&self) -> String {
        return match self {
            Self::Signal{message_id, signal_name, value_descriptions} => {
                format!("VAL_ {} {} \"{}\";",
                    message_id.dbc_string(),
                    signal_name,
                    value_descriptions
                        .clone()
                        .into_iter()
                        .map(|vd| vd.dbc_string())
                        .collect::<Vec<String>>()
                        .join(";") // TODO verify this is the correct delimiter
                )
            },
            Self::EnvironmentVariable{env_var_name, value_descriptions} => {
                format!("VAL_ {} \"{}\";",
                    env_var_name,
                    value_descriptions
                        .clone()
                        .into_iter()
                        .map(|vd| vd.dbc_string())
                        .collect::<Vec<String>>()
                        .join(";") // TODO verify this is the correct delimiter
                )
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SignalTypeRef {
    message_id: MessageId,
    signal_name: String,
    signal_type_name: String,
}

impl DBCString for SignalTypeRef {
    fn dbc_string(&self) -> String {
        return format!("SGTYPE_ {} {} : {};",
            self.message_id.dbc_string(),
            self.signal_name,
            self.signal_type_name,
        )
    }
}

/// Signal groups define a group of signals within a message
#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SignalGroups {
    message_id: MessageId,
    signal_group_name: String,
    repetitions: u64,
    signal_names: Vec<String>,
}

impl DBCString for SignalGroups {
    fn dbc_string(&self) -> String {
        return format!("SIG_GROUP_ {} {} {} : {};",
            self.message_id.dbc_string(),
            self.signal_group_name,
            self.repetitions,
            self.signal_names
                .join(" ")
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum SignalExtendedValueType {
    SignedOrUnsignedInteger,
    IEEEfloat32Bit,
    IEEEdouble64bit,
}

impl DBCString for SignalExtendedValueType {
    fn dbc_string(&self) -> String {
        return match self {
            Self::SignedOrUnsignedInteger => "0",
            Self::IEEEfloat32Bit => "1",
            Self::IEEEdouble64bit => "2",
        }.to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SignalExtendedValueTypeList {
    message_id: MessageId,
    signal_name: String,
    signal_extended_value_type: SignalExtendedValueType,
}

impl DBCString for SignalExtendedValueTypeList {
    fn dbc_string(&self) -> String {
        return format!("SIG_VALTYPE_ {} {}: {}",
            self.message_id.dbc_string(),
            self.signal_name,
            self.signal_extended_value_type.dbc_string(),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct DBC {
    /// Version generated by DB editor
    version: Version,
    new_symbols: Vec<Symbol>,
    /// Baud rate of network
    bit_timing: Option<Vec<Baudrate>>,
    /// CAN network nodes
    nodes: Vec<Node>,
    /// Global value table
    value_tables: Vec<ValueTable>,
    /// CAN message (frame) details including signal details
    messages: Vec<Message>,
    message_transmitters: Vec<MessageTransmitter>,
    environment_variables: Vec<EnvironmentVariable>,
    environment_variable_data: Vec<EnvironmentVariableData>,
    signal_types: Vec<SignalType>,
    /// Object comments
    comments: Vec<Comment>,
    attribute_definitions: Vec<AttributeDefinition>,
    // undefined
    // sigtype_attr_list: SigtypeAttrList,
    attribute_defaults: Vec<AttributeDefault>,
    attribute_values: Vec<AttributeValueForObject>,
    /// Encoding for signal raw values
    value_descriptions: Vec<ValueDescription>,
    // obsolete + undefined
    // category_definitions: Vec<CategoryDefinition>,
    // obsolete + undefined
    //categories: Vec<Category>,
    // obsolete + undefined
    //filter: Vec<Filter>,
    signal_type_refs: Vec<SignalTypeRef>,
    /// Signal groups define a group of signals within a message
    signal_groups: Vec<SignalGroups>,
    signal_extended_value_type_list: Vec<SignalExtendedValueTypeList>,
    /// Extended multiplex attributes
    extended_multiplex: Vec<ExtendedMultiplex>,
}

#[allow(dead_code)]
fn dbc_vec_to_string<T: DBCString>(dbc_objects: &Vec<T>, delimiter: &str) -> String {
    dbc_objects
    .into_iter()
    .map(|sym| sym.dbc_string())
    .collect::<Vec<String>>()
    .join(delimiter)
}

impl DBCString for DBC {
    fn dbc_string(&self) -> String {
        let mut file_str = String::new();
        // Version
        file_str += &self.version.dbc_string();

        // Symbols
        file_str += "NS_ :\n";
        file_str += &dbc_vec_to_string::<Symbol>(&self.new_symbols, "\n    ");
        

        // Baudrates
        match &self.bit_timing {
            Some(bauds) => {
                // confirm delineator
                file_str += &dbc_vec_to_string::<Baudrate>(&bauds, "\n");
            }
            None => {}
        }

        // Nodes
        file_str += &dbc_vec_to_string::<Node>(&self.nodes, " ");

        // Value Tables
        file_str += &dbc_vec_to_string::<ValueTable>(&self.value_tables, " ");

        // Messages
        file_str += &dbc_vec_to_string::<Message>(&self.messages, "\n");
        file_str += &dbc_vec_to_string::<MessageTransmitter>(&self.message_transmitters, " ");
        
        // Environment Variables
        file_str += &dbc_vec_to_string::<EnvironmentVariable>(&self.environment_variables, "\n");
        file_str += &dbc_vec_to_string::<EnvironmentVariableData>(&self.environment_variable_data, "\n");

        // Signal Types
        file_str += &dbc_vec_to_string::<SignalType>(&self.signal_types, " ");

        // Comments
        file_str += &dbc_vec_to_string::<Comment>(&self.comments, "\n");

        // Attributes
        file_str += &dbc_vec_to_string::<AttributeDefinition>(&self.attribute_definitions, "\n");
        file_str += &dbc_vec_to_string::<AttributeValueForObject>(&self.attribute_values, "\n");
        file_str += &dbc_vec_to_string::<AttributeDefault>(&self.attribute_defaults, "\n");

        // Value Descriptions
        file_str += &dbc_vec_to_string::<ValueDescription>(&self.value_descriptions, "\n");

        // Signal Attributes
        file_str += &dbc_vec_to_string::<SignalTypeRef>(&self.signal_type_refs, "\n");
        file_str += &dbc_vec_to_string::<SignalGroups>(&self.signal_groups, "\n");
        file_str += &dbc_vec_to_string::<SignalExtendedValueTypeList>(&self.signal_extended_value_type_list, "\n");

        // Multiplex
        file_str += &dbc_vec_to_string::<ExtendedMultiplex>(&self.extended_multiplex, "\n");

        return file_str
    }
}

impl DBC {
    /// Read a DBC from a buffer
    #[allow(clippy::result_large_err)]
    pub fn from_slice(buffer: &[u8]) -> Result<DBC, Error> {
        let dbc_in = std::str::from_utf8(buffer).unwrap();
        Self::try_from(dbc_in)
    }

    #[allow(clippy::should_implement_trait)]
    #[deprecated(since = "4.0.0", note = "please use `DBC::try_from` instead")]
    #[allow(clippy::result_large_err)]
    pub fn from_str(dbc_in: &str) -> Result<DBC, Error> {
        let (remaining, dbc) = parser::dbc(dbc_in).map_err(Error::Nom)?;
        if !remaining.is_empty() {
            return Err(Error::Incomplete(dbc, remaining));
        }
        Ok(dbc)
    }

    pub fn signal_by_name(&self, message_id: MessageId, signal_name: &str) -> Option<&Signal> {
        let message = self
            .messages
            .iter()
            .find(|message| message.message_id == message_id);

        if let Some(message) = message {
            return message
                .signals
                .iter()
                .find(|signal| signal.name == *signal_name);
        }
        None
    }

    /// Lookup a message comment
    pub fn message_comment(&self, message_id: MessageId) -> Option<&str> {
        self.comments
            .iter()
            .filter_map(|x| match x {
                Comment::Message {
                    message_id: ref x_message_id,
                    ref comment,
                } => {
                    if *x_message_id == message_id {
                        Some(comment.as_str())
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .next()
    }

    /// Lookup a signal comment
    pub fn signal_comment(&self, message_id: MessageId, signal_name: &str) -> Option<&str> {
        self.comments
            .iter()
            .filter_map(|x| match x {
                Comment::Signal {
                    message_id: ref x_message_id,
                    signal_name: ref x_signal_name,
                    comment,
                } => {
                    if *x_message_id == message_id && x_signal_name == signal_name {
                        Some(comment.as_str())
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .next()
    }

    /// Lookup value descriptions for signal
    pub fn value_descriptions_for_signal(
        &self,
        message_id: MessageId,
        signal_name: &str,
    ) -> Option<&[ValDescription]> {
        self.value_descriptions
            .iter()
            .filter_map(|x| match x {
                ValueDescription::Signal {
                    message_id: ref x_message_id,
                    signal_name: ref x_signal_name,
                    ref value_descriptions,
                } => {
                    if *x_message_id == message_id && x_signal_name == signal_name {
                        Some(value_descriptions.as_slice())
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .next()
    }

    /// Lookup the extended value for a given signal
    pub fn extended_value_type_for_signal(
        &self,
        message_id: MessageId,
        signal_name: &str,
    ) -> Option<&SignalExtendedValueType> {
        self.signal_extended_value_type_list
            .iter()
            .filter_map(|x| {
                let SignalExtendedValueTypeList {
                    message_id: ref x_message_id,
                    signal_name: ref x_signal_name,
                    ref signal_extended_value_type,
                } = x;
                if *x_message_id == message_id && x_signal_name == signal_name {
                    Some(signal_extended_value_type)
                } else {
                    None
                }
            })
            .next()
    }

    /// Lookup the message multiplexor switch signal for a given message
    /// This does not work for extended multiplexed messages, if multiple multiplexors are defined for a message a Error is returned.
    #[allow(clippy::result_large_err)]
    pub fn message_multiplexor_switch(
        &self,
        message_id: MessageId,
    ) -> Result<Option<&Signal>, Error> {
        let message = self
            .messages
            .iter()
            .find(|message| message.message_id == message_id);

        if let Some(message) = message {
            if self
                .extended_multiplex
                .iter()
                .any(|ext_mp| ext_mp.message_id == message_id)
            {
                Err(Error::MultipleMultiplexors)
            } else {
                Ok(message
                    .signals
                    .iter()
                    .find(|signal| signal.multiplexer_indicator == MultiplexIndicator::Multiplexor))
            }
        } else {
            Ok(None)
        }
    }
}

impl<'a> TryFrom<&'a str> for DBC {
    type Error = Error<'a>;

    fn try_from(dbc_in: &'a str) -> Result<Self, Self::Error> {
        let (remaining, dbc) = parser::dbc(dbc_in).map_err(Error::Nom)?;
        if !remaining.is_empty() {
            return Err(Error::Incomplete(dbc, remaining));
        }
        Ok(dbc)
    }
}
