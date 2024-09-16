#[cfg(feature = "with-serde")]
extern crate serde;
#[cfg(feature = "with-serde")]
#[macro_use]
extern crate serde_derive;


use derive_getters::Getters;

use crate::DBCString;
use crate::message::MessageId;

/// One or multiple signals are the payload of a CAN frame.
/// To determine the actual value of a signal the following fn applies:
/// `let fnvalue = |can_signal_value| -> can_signal_value * factor + offset;`
#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Signal {
    pub (crate) name: String,
    pub (crate) multiplexer_indicator: MultiplexIndicator,
    pub start_bit: u64,
    pub signal_size: u64,
    pub (crate) byte_order: ByteOrder,
    pub (crate) value_type: ValueType,
    pub factor: f64,
    pub offset: f64,
    pub min: f64,
    pub max: f64,
    pub (crate) unit: String,
    pub (crate) receivers: Vec<String>,
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


#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SignalType {
    pub (crate) signal_type_name: String,
    pub (crate) signal_size: u64,
    pub (crate) byte_order: ByteOrder,
    pub (crate) value_type: ValueType,
    pub (crate) factor: f64,
    pub (crate) offset: f64,
    pub (crate) min: f64,
    pub (crate) max: f64,
    pub (crate) unit: String,
    pub (crate) default_value: f64,
    pub (crate) value_table: String,
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

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ExtendedMultiplexMapping {
    pub (crate) min_value: u64,
    pub (crate) max_value: u64,
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
    pub message_id: MessageId,
    pub signal_name: String,
    pub multiplexor_signal_name: String,
    pub mappings: Vec<ExtendedMultiplexMapping>,
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


/// Signal groups define a group of signals within a message
#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SignalGroups {
    pub (crate) message_id: MessageId,
    pub (crate) signal_group_name: String,
    pub (crate) repetitions: u64,
    pub (crate) signal_names: Vec<String>,
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
    pub message_id: MessageId,
    pub signal_name: String,
    pub signal_extended_value_type: SignalExtendedValueType,
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
pub struct SignalTypeRef {
    pub (crate) message_id: MessageId,
    pub (crate) signal_name: String,
    pub (crate) signal_type_name: String,
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
