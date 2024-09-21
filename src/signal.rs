#[cfg(feature = "with-serde")]
extern crate serde;
#[cfg(feature = "with-serde")]
#[macro_use]
extern crate serde_derive;

use derive_getters::Getters;

use crate::parser;
use crate::DBCObject;
use crate::MessageId;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, char, line_ending, multispace0},
    combinator::{map, opt, value},
    multi::separated_list0,
    number::complete::double,
    IResult,
};

/// One or multiple signals are the payload of a CAN frame.
/// To determine the actual value of a signal the following fn applies:
/// `let fnvalue = |can_signal_value| -> can_signal_value * factor + offset;`
#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Signal {
    pub(crate) name: String,
    pub(crate) multiplexer_indicator: MultiplexIndicator,
    pub start_bit: u64,
    pub signal_size: u64,
    pub(crate) byte_order: ByteOrder,
    pub(crate) value_type: ValueType,
    pub factor: f64,
    pub offset: f64,
    pub min: f64,
    pub max: f64,
    pub(crate) unit: String,
    pub(crate) receivers: Vec<String>,
}

impl DBCObject for Signal {
    fn dbc_string(&self) -> String {
        let receivers = match self.receivers.len() {
            0 => "VECTOR_XXX".to_string(),
            _ => self.receivers.join(", "),
        };
        // format! macro doesn't support direct field access inline with the string
        return format!(
            "SG_ {} {}: {}|{}@{}{} ({},{}) [{}|{}] \"{}\" {}\n",
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
        );
    }

    fn parse(s: &str) -> nom::IResult<&str, Self>
    where
        Self: Sized,
    {
        let (s, _) = multispace0(s)?;
        let (s, _) = tag("SG_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, name) = parser::c_ident(s)?;
        let (s, multiplexer_indicator) = MultiplexIndicator::parse(s)?;
        let (s, _) = parser::colon(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, start_bit) = complete::u64(s)?;
        let (s, _) = parser::pipe(s)?;
        let (s, signal_size) = complete::u64(s)?;
        let (s, _) = parser::at(s)?;
        let (s, byte_order) = ByteOrder::parse(s)?;
        let (s, value_type) = ValueType::parse(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, _) = parser::brc_open(s)?;
        let (s, factor) = double(s)?;
        let (s, _) = parser::comma(s)?;
        let (s, offset) = double(s)?;
        let (s, _) = parser::brc_close(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, _) = parser::brk_open(s)?;
        let (s, min) = double(s)?;
        let (s, _) = parser::pipe(s)?;
        let (s, max) = double(s)?;
        let (s, _) = parser::brk_close(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, unit) = parser::char_string(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, receivers) = parser::c_ident_vec(s)?;
        let (s, _) = line_ending(s)?;
        Ok((
            s,
            Signal {
                name,
                multiplexer_indicator,
                start_bit,
                signal_size,
                byte_order,
                value_type,
                factor,
                offset,
                min,
                max,
                unit: unit.to_string(),
                receivers,
            },
        ))
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

impl MultiplexIndicator {
    fn multiplexer(s: &str) -> IResult<&str, MultiplexIndicator> {
        let (s, _) = parser::ms1(s)?;
        let (s, _) = char('m')(s)?;
        let (s, d) = complete::u64(s)?;
        let (s, _) = parser::ms1(s)?;
        Ok((s, MultiplexIndicator::MultiplexedSignal(d)))
    }

    fn multiplexor(s: &str) -> IResult<&str, MultiplexIndicator> {
        let (s, _) = parser::ms1(s)?;
        let (s, _) = char('M')(s)?;
        let (s, _) = parser::ms1(s)?;
        Ok((s, MultiplexIndicator::Multiplexor))
    }

    fn multiplexor_and_multiplexed(s: &str) -> IResult<&str, MultiplexIndicator> {
        let (s, _) = parser::ms1(s)?;
        let (s, _) = char('m')(s)?;
        let (s, d) = complete::u64(s)?;
        let (s, _) = char('M')(s)?;
        let (s, _) = parser::ms1(s)?;
        Ok((s, MultiplexIndicator::MultiplexorAndMultiplexedSignal(d)))
    }

    fn plain(s: &str) -> IResult<&str, MultiplexIndicator> {
        let (s, _) = parser::ms1(s)?;
        Ok((s, MultiplexIndicator::Plain))
    }
}

impl DBCObject for MultiplexIndicator {
    fn dbc_string(&self) -> String {
        return match self {
            Self::Multiplexor => "M ".to_string(),
            Self::MultiplexedSignal(m) => format!("m{m} ").to_string(),
            Self::MultiplexorAndMultiplexedSignal(m) => format!("M{m} ").to_string(),
            Self::Plain => "".to_string(),
        };
    }

    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        alt((
            Self::multiplexer,
            Self::multiplexor,
            Self::multiplexor_and_multiplexed,
            Self::plain,
        ))(s)
    }
}

#[test]
fn multiplexer_indicator_parse_test() {
    // TODO fix the space padding to make this more robust
    let (_, multiplexer) =
        MultiplexIndicator::parse(" m34920 ").expect("Failed to parse multiplexer");
    assert_eq!(MultiplexIndicator::MultiplexedSignal(34920), multiplexer);

    let (_, multiplexor) = MultiplexIndicator::parse(" M ").expect("Failed to parse multiplexor");
    assert_eq!(MultiplexIndicator::Multiplexor, multiplexor);

    let (_, plain) = MultiplexIndicator::parse(" ").expect("Failed to parse plain");
    assert_eq!(MultiplexIndicator::Plain, plain);

    let (_, multiplexer) = MultiplexIndicator::parse(" m8M ").expect("Failed to parse multiplexer");
    assert_eq!(
        MultiplexIndicator::MultiplexorAndMultiplexedSignal(8),
        multiplexer
    );
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum ByteOrder {
    LittleEndian,
    BigEndian,
}

impl ByteOrder {
    pub(crate) fn little_endian(s: &str) -> IResult<&str, ByteOrder> {
        map(char('1'), |_| ByteOrder::LittleEndian)(s)
    }

    pub(crate) fn big_endian(s: &str) -> IResult<&str, ByteOrder> {
        map(char('0'), |_| ByteOrder::BigEndian)(s)
    }
}

impl DBCObject for ByteOrder {
    fn dbc_string(&self) -> String {
        return match self {
            Self::LittleEndian => "1".to_string(),
            Self::BigEndian => "0".to_string(),
        };
    }

    fn parse(s: &str) -> nom::IResult<&str, Self>
    where
        Self: Sized,
    {
        alt((Self::little_endian, Self::big_endian))(s)
    }
}

#[test]
fn byte_order_test() {
    let def = "0";
    let (_, big_endian) = ByteOrder::parse(def).expect("Failed to parse big endian");

    // Test parsing
    assert_eq!(ByteOrder::BigEndian, big_endian);

    // Test generation
    assert_eq!(def, big_endian.dbc_string());

    let def = "1";
    let (_, little_endian) = ByteOrder::parse(def).expect("Failed to parse little endian");

    // Test parsing
    assert_eq!(ByteOrder::LittleEndian, little_endian);

    // Test generation
    assert_eq!(def, little_endian.dbc_string());
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum ValueType {
    Signed,
    Unsigned,
}

impl ValueType {
    fn signed(s: &str) -> IResult<&str, ValueType> {
        map(char('-'), |_| ValueType::Signed)(s)
    }

    fn unsigned(s: &str) -> IResult<&str, ValueType> {
        map(char('+'), |_| ValueType::Unsigned)(s)
    }
}

impl DBCObject for ValueType {
    fn dbc_string(&self) -> String {
        return match self {
            Self::Signed => "-".to_string(),
            Self::Unsigned => "+".to_string(),
        };
    }

    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        alt((Self::signed, Self::unsigned))(s)
    }
}

#[test]
fn value_type_test() {
    let def = "- ";
    let (_, vt) = ValueType::parse(def).expect("Failed to parse value type");

    // Test parsing
    assert_eq!(ValueType::Signed, vt);

    // Test generation
    assert_eq!(def.trim(), vt.dbc_string());

    let def = "+ ";
    let (_, vt) = ValueType::parse(def).expect("Failed to parse value type");

    // Test parsing
    assert_eq!(ValueType::Unsigned, vt);

    // Test generation
    assert_eq!(def.trim(), vt.dbc_string());
}

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SignalType {
    pub(crate) signal_type_name: String,
    pub(crate) signal_size: u64,
    pub(crate) byte_order: ByteOrder,
    pub(crate) value_type: ValueType,
    pub(crate) factor: f64,
    pub(crate) offset: f64,
    pub(crate) min: f64,
    pub(crate) max: f64,
    pub(crate) unit: String,
    pub(crate) default_value: f64,
    pub(crate) value_table: String,
}

impl DBCObject for SignalType {
    fn dbc_string(&self) -> String {
        // TODO this is difficult to test since CANdb++ doesn't seem to have this feature implemented
        return format!(
            "SGTYPE_ {}: {}@{}{} ({},{}) [{}|{}] \"{}\" {} {};\n",
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
        );
    }

    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        let (s, _) = multispace0(s)?;
        let (s, _) = tag("SGTYPE_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, signal_type_name) = parser::c_ident(s)?;
        let (s, _) = parser::colon(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, signal_size) = complete::u64(s)?;
        let (s, _) = parser::at(s)?;
        let (s, byte_order) = ByteOrder::parse(s)?;
        let (s, value_type) = ValueType::parse(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, _) = parser::brc_open(s)?;
        let (s, factor) = double(s)?;
        let (s, _) = parser::comma(s)?;
        let (s, offset) = double(s)?;
        let (s, _) = parser::brc_close(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, _) = parser::brk_open(s)?;
        let (s, min) = double(s)?;
        let (s, _) = parser::pipe(s)?;
        let (s, max) = double(s)?;
        let (s, _) = parser::brk_close(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, unit) = parser::char_string(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, default_value) = double(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, value_table) = parser::c_ident(s)?;
        let (s, _) = parser::semi_colon(s)?;
        let (s, _) = line_ending(s)?;
        Ok((
            s,
            SignalType {
                signal_type_name,
                signal_size,
                byte_order,
                value_type,
                factor,
                offset,
                min,
                max,
                unit: unit.to_string(),
                default_value,
                value_table,
            },
        ))
    }
}

#[test]
fn signal_type_test() {
    let def = "SGTYPE_ signal_type_name: 1024@1+ (5,2) [1|3] \"unit\" 2 val_table;\n";

    let exp = SignalType {
        signal_type_name: "signal_type_name".to_string(),
        signal_size: 1024,
        byte_order: ByteOrder::LittleEndian,
        value_type: ValueType::Unsigned,
        factor: 5.0,
        offset: 2.0,
        min: 1.0,
        max: 3.0,
        unit: "unit".to_string(),
        default_value: 2.0,
        value_table: "val_table".to_string(),
    };

    let (_, signal_type) = SignalType::parse(def).unwrap();

    // Test parsing
    assert_eq!(exp, signal_type);

    // Test generation
    assert_eq!(def, signal_type.dbc_string());
}

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ExtendedMultiplexMapping {
    pub(crate) min_value: u64,
    pub(crate) max_value: u64,
}

impl DBCObject for ExtendedMultiplexMapping {
    fn dbc_string(&self) -> String {
        return format!("{}-{}", self.min_value, self.max_value);
    }

    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        let (s, _) = parser::ms0(s)?;
        let (s, min_value) = complete::u64(s)?;
        let (s, _) = char('-')(s)?;
        let (s, max_value) = complete::u64(s)?;
        Ok((
            s,
            ExtendedMultiplexMapping {
                min_value,
                max_value,
            },
        ))
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

impl DBCObject for ExtendedMultiplex {
    fn dbc_string(&self) -> String {
        return format!(
            "SG_MUL_VAL_ {} {} {} {};\n",
            self.message_id.dbc_string(),
            self.signal_name,
            self.multiplexor_signal_name,
            self.mappings
                .clone()
                .into_iter()
                .map(|m| m.dbc_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
    }

    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        let (s, _) = multispace0(s)?;
        let (s, _) = tag("SG_MUL_VAL_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, message_id) = MessageId::parse(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, signal_name) = parser::c_ident(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, multiplexor_signal_name) = parser::c_ident(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, mappings) = separated_list0(tag(","), ExtendedMultiplexMapping::parse)(s)?;
        let (s, _) = parser::semi_colon(s)?;
        let (s, _) = line_ending(s)?;
        Ok((
            s,
            ExtendedMultiplex {
                message_id,
                signal_name,
                multiplexor_signal_name,
                mappings,
            },
        ))
    }
}

#[test]
fn extended_multiplex_test() {
    // simple mapping
    let def = "SG_MUL_VAL_ 2147483650 muxed_A_1 MUX_A 1-1;\n";
    let exp = ExtendedMultiplex {
        message_id: MessageId::Extended(2),
        signal_name: "muxed_A_1".to_string(),
        multiplexor_signal_name: "MUX_A".to_string(),
        mappings: vec![ExtendedMultiplexMapping {
            min_value: 1,
            max_value: 1,
        }],
    };
    let (_, ext_multiplex) = ExtendedMultiplex::parse(def).unwrap();

    // Test parsing
    assert_eq!(exp, ext_multiplex);

    // Test generation
    assert_eq!(def, ext_multiplex.dbc_string());

    // range mapping
    let def = "SG_MUL_VAL_ 2147483650 muxed_A_1 MUX_A 1568-2568;\n";
    let exp = ExtendedMultiplex {
        message_id: MessageId::Extended(2),
        signal_name: "muxed_A_1".to_string(),
        multiplexor_signal_name: "MUX_A".to_string(),
        mappings: vec![ExtendedMultiplexMapping {
            min_value: 1568,
            max_value: 2568,
        }],
    };
    let (_, ext_multiplex) = ExtendedMultiplex::parse(def).unwrap();

    // Test parsing
    assert_eq!(exp, ext_multiplex);

    // Test generation
    assert_eq!(def, ext_multiplex.dbc_string());

    // multiple mappings
    let def = "SG_MUL_VAL_ 2147483650 muxed_B_5 MUX_B 5-5, 16-24;\n";
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
    let (_, ext_multiplex) = ExtendedMultiplex::parse(def).unwrap();

    // Test parsing
    assert_eq!(exp, ext_multiplex);

    // Test generation
    assert_eq!(def, ext_multiplex.dbc_string());
}

/// Signal groups define a group of signals within a message
#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SignalGroups {
    pub(crate) message_id: MessageId,
    pub(crate) signal_group_name: String,
    pub(crate) repetitions: u64,
    pub(crate) signal_names: Vec<String>,
}

impl DBCObject for SignalGroups {
    fn dbc_string(&self) -> String {
        return format!(
            "SIG_GROUP_ {} {} {} : {};\n",
            self.message_id.dbc_string(),
            self.signal_group_name,
            self.repetitions,
            self.signal_names.join(" ")
        );
    }

    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        let (s, _) = multispace0(s)?;
        let (s, _) = tag("SIG_GROUP_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, message_id) = MessageId::parse(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, signal_group_name) = parser::c_ident(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, repetitions) = complete::u64(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, _) = parser::colon(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, signal_names) = separated_list0(parser::ms1, parser::c_ident)(s)?;
        let (s, _) = parser::semi_colon(s)?;
        let (s, _) = line_ending(s)?;
        Ok((
            s,
            SignalGroups {
                message_id,
                signal_group_name,
                repetitions,
                signal_names,
            },
        ))
    }
}

#[test]
fn signal_groups_test() {
    let def = "SIG_GROUP_ 23 X_3290 1 : A_b XY_Z;\n";

    let exp = SignalGroups {
        message_id: MessageId::Standard(23),
        signal_group_name: "X_3290".to_string(),
        repetitions: 1,
        signal_names: vec!["A_b".to_string(), "XY_Z".to_string()],
    };

    let (_, signal_groups) = SignalGroups::parse(def).unwrap();

    // Test parsing
    assert_eq!(exp, signal_groups);

    // Test generation
    assert_eq!(def, signal_groups.dbc_string());
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum SignalExtendedValueType {
    SignedOrUnsignedInteger,
    IEEEfloat32Bit,
    IEEEdouble64bit,
}

impl SignalExtendedValueType {
    fn signed_or_unsigned_integer(s: &str) -> IResult<&str, SignalExtendedValueType> {
        value(SignalExtendedValueType::SignedOrUnsignedInteger, tag("0"))(s)
    }
    fn ieee_float_32bit(s: &str) -> IResult<&str, SignalExtendedValueType> {
        value(SignalExtendedValueType::IEEEfloat32Bit, tag("1"))(s)
    }
    fn ieee_double_64bit(s: &str) -> IResult<&str, SignalExtendedValueType> {
        value(SignalExtendedValueType::IEEEdouble64bit, tag("2"))(s)
    }
}

impl DBCObject for SignalExtendedValueType {
    fn dbc_string(&self) -> String {
        return match self {
            Self::SignedOrUnsignedInteger => "0",
            Self::IEEEfloat32Bit => "1",
            Self::IEEEdouble64bit => "2",
        }
        .to_string();
    }

    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        alt((
            Self::signed_or_unsigned_integer,
            Self::ieee_float_32bit,
            Self::ieee_double_64bit,
        ))(s)
    }
}

#[test]
fn sig_val_type_test() {
    let def = "SIG_VALTYPE_ 2000 Signal_8 : 1;\n";
    let exp = SignalExtendedValueTypeList {
        message_id: MessageId::Standard(2000),
        signal_name: "Signal_8".to_string(),
        signal_extended_value_type: SignalExtendedValueType::IEEEfloat32Bit,
    };

    let (_, extended_value_type_list) = SignalExtendedValueTypeList::parse(def).unwrap();

    // Test parsing
    assert_eq!(extended_value_type_list, exp);

    // Test generation
    assert_eq!(def, extended_value_type_list.dbc_string());
}

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SignalExtendedValueTypeList {
    pub message_id: MessageId,
    pub signal_name: String,
    pub signal_extended_value_type: SignalExtendedValueType,
}

impl DBCObject for SignalExtendedValueTypeList {
    fn dbc_string(&self) -> String {
        return format!(
            "SIG_VALTYPE_ {} {} : {};\n",
            self.message_id.dbc_string(),
            self.signal_name,
            self.signal_extended_value_type.dbc_string(),
        );
    }

    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        let (s, _) = multispace0(s)?;
        let (s, _) = tag("SIG_VALTYPE_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, message_id) = MessageId::parse(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, signal_name) = parser::c_ident(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, _) = opt(parser::colon)(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, signal_extended_value_type) = SignalExtendedValueType::parse(s)?;
        let (s, _) = parser::semi_colon(s)?;
        let (s, _) = line_ending(s)?;
        Ok((
            s,
            SignalExtendedValueTypeList {
                message_id,
                signal_name,
                signal_extended_value_type,
            },
        ))
    }
}

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SignalTypeRef {
    pub(crate) message_id: MessageId,
    pub(crate) signal_name: String,
    pub(crate) signal_type_name: String,
}

impl DBCObject for SignalTypeRef {
    fn dbc_string(&self) -> String {
        return format!(
            "SGTYPE_ {} {} : {};",
            self.message_id.dbc_string(),
            self.signal_name,
            self.signal_type_name,
        );
    }

    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        let (s, _) = multispace0(s)?;
        let (s, _) = tag("SGTYPE_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, message_id) = MessageId::parse(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, signal_name) = parser::c_ident(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, _) = parser::colon(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, signal_type_name) = parser::c_ident(s)?;
        let (s, _) = parser::semi_colon(s)?;
        let (s, _) = line_ending(s)?;
        Ok((
            s,
            SignalTypeRef {
                message_id,
                signal_name,
                signal_type_name,
            },
        ))
    }
}
