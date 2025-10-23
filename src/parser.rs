//!
//! Module containing nom parser combinators
//!

use std::str;

use nom::branch::{alt, permutation};
use nom::bytes::complete::{escaped, tag, take_till, take_till1, take_while, take_while1};
use nom::character::complete::{self, char, line_ending, multispace0, one_of, space0, space1};
use nom::combinator::{map, opt, value};
use nom::error::{ErrorKind, ParseError};
use nom::multi::{many0, many_till, separated_list0};
use nom::number::complete::double;
use nom::sequence::preceded;
use nom::{AsChar, IResult, Input, Parser};

use crate::{
    AccessNode, AccessType, AttributeDefault, AttributeDefinition, AttributeValue,
    AttributeValueForObject, AttributeValuedForObjectType, Baudrate, ByteOrder, Comment, Dbc,
    EnvType, EnvironmentVariable, EnvironmentVariableData, ExtendedMultiplex,
    ExtendedMultiplexMapping, Message, MessageId, MessageTransmitter, MultiplexIndicator, Node,
    Signal, SignalExtendedValueType, SignalExtendedValueTypeList, SignalGroups, SignalType,
    SignalTypeRef, Symbol, Transmitter, ValDescription, ValueDescription, ValueTable, ValueType,
    Version,
};

fn is_semi_colon(chr: char) -> bool {
    chr == ';'
}

fn is_c_string_char(chr: char) -> bool {
    chr.is_ascii_digit() || chr.is_alphabetic() || chr == '_'
}

fn is_c_ident_head(chr: char) -> bool {
    chr.is_alphabetic() || chr == '_'
}

fn is_quote_or_escape_character(chr: char) -> bool {
    chr == '"' || chr == '\\'
}

/// Multispace zero or more
fn ms0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: Input,
    <T as Input>::Item: AsChar + Clone,
{
    input.split_at_position_complete(|item| {
        let c = item.as_char();
        c != ' '
    })
}

/// Multi space one or more
fn ms1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: Input,
    <T as Input>::Item: AsChar + Clone,
{
    input.split_at_position1_complete(
        |item| {
            let c = item.as_char();
            c != ' '
        },
        ErrorKind::MultiSpace,
    )
}

/// Colon aka `:`
fn colon(s: &str) -> IResult<&str, char> {
    char(':').parse(s)
}

/// Comma aka ','
fn comma(s: &str) -> IResult<&str, char> {
    char(',').parse(s)
}

/// Comma aka ';'
fn semi_colon(s: &str) -> IResult<&str, char> {
    char(';').parse(s)
}

/// Quote aka '"'
fn quote(s: &str) -> IResult<&str, char> {
    char('"').parse(s)
}

/// Pipe character
fn pipe(s: &str) -> IResult<&str, char> {
    char('|').parse(s)
}

/// at character
fn at(s: &str) -> IResult<&str, char> {
    char('@').parse(s)
}

/// brace open aka '('
fn brc_open(s: &str) -> IResult<&str, char> {
    char('(').parse(s)
}

/// brace close aka ')'
fn brc_close(s: &str) -> IResult<&str, char> {
    char(')').parse(s)
}

/// bracket open aka '['
fn brk_open(s: &str) -> IResult<&str, char> {
    char('[').parse(s)
}

/// bracket close aka ']'
fn brk_close(s: &str) -> IResult<&str, char> {
    char(']').parse(s)
}

/// A valid `C_identifier`. `C_identifier`s start with an alpha character or an underscore
/// and may further consist of alphanumeric characters and underscore
pub(crate) fn c_ident(s: &str) -> IResult<&str, String> {
    let (s, head) = take_while1(is_c_ident_head).parse(s)?;
    let (s, remaining) = take_while(is_c_string_char).parse(s)?;
    Ok((s, [head, remaining].concat()))
}

pub(crate) fn c_ident_vec(s: &str) -> IResult<&str, Vec<String>> {
    separated_list0(comma, c_ident).parse(s)
}

pub(crate) fn char_string(s: &str) -> IResult<&str, &str> {
    let (s, _) = quote(s)?;
    let (s, optional_char_string_value) = opt(escaped(
        take_till1(is_quote_or_escape_character),
        '\\',
        one_of(r#""n\"#),
    ))
    .parse(s)?;
    let (s, _) = quote(s)?;

    let char_string_value = optional_char_string_value.unwrap_or("");
    Ok((s, char_string_value))
}

fn little_endian(s: &str) -> IResult<&str, ByteOrder> {
    map(char('1'), |_| ByteOrder::LittleEndian).parse(s)
}

fn big_endian(s: &str) -> IResult<&str, ByteOrder> {
    map(char('0'), |_| ByteOrder::BigEndian).parse(s)
}

pub(crate) fn byte_order(s: &str) -> IResult<&str, ByteOrder> {
    alt((little_endian, big_endian)).parse(s)
}

pub(crate) fn message_id(s: &str) -> IResult<&str, MessageId> {
    let (s, parsed_value) = complete::u32(s)?;

    if parsed_value & (1 << 31) != 0 {
        Ok((s, MessageId::Extended(parsed_value & 0x1FFF_FFFF)))
    } else {
        // FIXME: use u16::try_from and handle error
        #[allow(clippy::cast_possible_truncation)]
        Ok((s, MessageId::Standard(parsed_value as u16)))
    }
}

fn signed(s: &str) -> IResult<&str, ValueType> {
    map(char('-'), |_| ValueType::Signed).parse(s)
}

fn unsigned(s: &str) -> IResult<&str, ValueType> {
    map(char('+'), |_| ValueType::Unsigned).parse(s)
}

pub(crate) fn value_type(s: &str) -> IResult<&str, ValueType> {
    alt((signed, unsigned)).parse(s)
}

fn multiplexer(s: &str) -> IResult<&str, MultiplexIndicator> {
    let (s, _) = ms1(s)?;
    let (s, _) = char('m').parse(s)?;
    let (s, d) = complete::u64(s)?;
    let (s, _) = ms1(s)?;
    Ok((s, MultiplexIndicator::MultiplexedSignal(d)))
}

fn multiplexor(s: &str) -> IResult<&str, MultiplexIndicator> {
    let (s, _) = ms1(s)?;
    let (s, _) = char('M').parse(s)?;
    let (s, _) = ms1(s)?;
    Ok((s, MultiplexIndicator::Multiplexor))
}

fn multiplexor_and_multiplexed(s: &str) -> IResult<&str, MultiplexIndicator> {
    let (s, _) = ms1(s)?;
    let (s, _) = char('m').parse(s)?;
    let (s, d) = complete::u64(s)?;
    let (s, _) = char('M').parse(s)?;
    let (s, _) = ms1(s)?;
    Ok((s, MultiplexIndicator::MultiplexorAndMultiplexedSignal(d)))
}

fn plain(s: &str) -> IResult<&str, MultiplexIndicator> {
    let (s, _) = ms1(s)?;
    Ok((s, MultiplexIndicator::Plain))
}

pub(crate) fn multiplexer_indicator(s: &str) -> IResult<&str, MultiplexIndicator> {
    alt((multiplexer, multiplexor, multiplexor_and_multiplexed, plain)).parse(s)
}

pub(crate) fn version(s: &str) -> IResult<&str, Version> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("VERSION").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, v) = char_string(s)?;
    let (s, _) = line_ending(s)?;
    Ok((s, Version(v.to_string())))
}

fn bit_timing(s: &str) -> IResult<&str, Vec<Baudrate>> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("BS_:").parse(s)?;
    let (s, baudrates) = opt(preceded(
        ms1,
        separated_list0(comma, map(complete::u64, Baudrate)),
    ))
    .parse(s)?;
    Ok((s, baudrates.unwrap_or_default()))
}

pub(crate) fn signal(s: &str) -> IResult<&str, Signal> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("SG_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, name) = c_ident(s)?;
    let (s, multiplexer_indicator) = multiplexer_indicator(s)?;
    let (s, _) = colon(s)?;
    let (s, _) = ms1(s)?;
    let (s, start_bit) = complete::u64(s)?;
    let (s, _) = pipe(s)?;
    let (s, signal_size) = complete::u64(s)?;
    let (s, _) = at(s)?;
    let (s, byte_order) = byte_order(s)?;
    let (s, value_type) = value_type(s)?;
    let (s, _) = ms1(s)?;
    let (s, _) = brc_open(s)?;
    let (s, factor) = double(s)?;
    let (s, _) = comma(s)?;
    let (s, offset) = double(s)?;
    let (s, _) = brc_close(s)?;
    let (s, _) = ms1(s)?;
    let (s, _) = brk_open(s)?;
    let (s, min) = double(s)?;
    let (s, _) = pipe(s)?;
    let (s, max) = double(s)?;
    let (s, _) = brk_close(s)?;
    let (s, _) = ms1(s)?;
    let (s, unit) = char_string(s)?;
    let (s, _) = ms1(s)?;
    let (s, receivers) = c_ident_vec(s)?;
    let (s, _) = line_ending(s)?;
    Ok((
        s,
        Signal {
            name,
            multiplexer_indicator,
            start_bit,
            size: signal_size,
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

pub(crate) fn message(s: &str) -> IResult<&str, Message> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("BO_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, message_id) = message_id(s)?;
    let (s, _) = ms1(s)?;
    let (s, message_name) = c_ident(s)?;
    let (s, _) = colon(s)?;
    let (s, _) = ms1(s)?;
    let (s, message_size) = complete::u64(s)?;
    let (s, _) = ms1(s)?;
    let (s, transmitter) = transmitter(s)?;
    let (s, signals) = many0(signal).parse(s)?;
    Ok((
        s,
        (Message {
            id: message_id,
            name: message_name,
            size: message_size,
            transmitter,
            signals,
        }),
    ))
}

pub(crate) fn attribute_default(s: &str) -> IResult<&str, AttributeDefault> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("BA_DEF_DEF_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, attribute_name) = char_string(s)?;
    let (s, _) = ms1(s)?;
    let (s, attribute_value) = attribute_value(s)?;
    let (s, _) = semi_colon(s)?;
    let (s, _) = line_ending(s)?;

    Ok((
        s,
        AttributeDefault {
            name: attribute_name.to_string(),
            value: attribute_value,
        },
    ))
}

fn node_comment(s: &str) -> IResult<&str, Comment> {
    let (s, _) = tag("BU_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, node_name) = c_ident(s)?;
    let (s, _) = ms1(s)?;
    let (s, comment) = char_string(s)?;

    Ok((
        s,
        Comment::Node {
            name: node_name,
            comment: comment.to_string(),
        },
    ))
}

fn message_comment(s: &str) -> IResult<&str, Comment> {
    let (s, _) = tag("BO_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, message_id) = message_id(s)?;
    let (s, _) = ms1(s)?;
    let (s, comment) = char_string(s)?;

    Ok((
        s,
        Comment::Message {
            id: message_id,
            comment: comment.to_string(),
        },
    ))
}

fn signal_comment(s: &str) -> IResult<&str, Comment> {
    let (s, _) = tag("SG_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, message_id) = message_id(s)?;
    let (s, _) = ms1(s)?;
    let (s, signal_name) = c_ident(s)?;
    let (s, _) = ms1(s)?;
    let (s, comment) = char_string(s)?;
    Ok((
        s,
        Comment::Signal {
            message_id,
            name: signal_name,
            comment: comment.to_string(),
        },
    ))
}

fn env_var_comment(s: &str) -> IResult<&str, Comment> {
    let (s, _) = ms0(s)?;
    let (s, _) = tag("EV_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, env_var_name) = c_ident(s)?;
    let (s, _) = ms1(s)?;
    let (s, comment) = char_string(s)?;
    Ok((
        s,
        Comment::EnvVar {
            name: env_var_name,
            comment: comment.to_string(),
        },
    ))
}

fn comment_plain(s: &str) -> IResult<&str, Comment> {
    let (s, comment) = char_string(s)?;
    Ok((
        s,
        Comment::Plain {
            comment: comment.to_string(),
        },
    ))
}

pub(crate) fn comment(s: &str) -> IResult<&str, Comment> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("CM_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, comment) = alt((
        node_comment,
        message_comment,
        env_var_comment,
        signal_comment,
        comment_plain,
    ))
    .parse(s)?;
    let (s, _) = semi_colon(s)?;
    let (s, _) = line_ending(s)?;
    Ok((s, comment))
}

pub(crate) fn value_description(s: &str) -> IResult<&str, ValDescription> {
    let (s, a) = double(s)?;
    let (s, _) = ms1(s)?;
    let (s, b) = char_string(s)?;
    Ok((
        s,
        ValDescription {
            id: a,
            description: b.to_string(),
        },
    ))
}

fn value_description_for_signal(s: &str) -> IResult<&str, ValueDescription> {
    let (s, _) = ms0(s)?;
    let (s, _) = tag("VAL_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, message_id) = message_id(s)?;
    let (s, _) = ms1(s)?;
    let (s, signal_name) = c_ident(s)?;
    let (s, value_descriptions) = many_till(
        preceded(ms1, value_description),
        preceded(opt(ms1), semi_colon),
    )
    .parse(s)?;
    Ok((
        s,
        ValueDescription::Signal {
            message_id,
            name: signal_name,
            value_descriptions: value_descriptions.0,
        },
    ))
}

fn value_description_for_env_var(s: &str) -> IResult<&str, ValueDescription> {
    let (s, _) = ms0(s)?;
    let (s, _) = tag("VAL_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, env_var_name) = c_ident(s)?;
    let (s, value_descriptions) = many_till(
        preceded(ms1, value_description),
        preceded(opt(ms1), semi_colon),
    )
    .parse(s)?;
    Ok((
        s,
        ValueDescription::EnvironmentVariable {
            name: env_var_name,
            value_descriptions: value_descriptions.0,
        },
    ))
}

pub(crate) fn value_descriptions(s: &str) -> IResult<&str, ValueDescription> {
    let (s, _) = multispace0(s)?;
    let (s, vd) = alt((value_description_for_signal, value_description_for_env_var)).parse(s)?;
    let (s, _) = line_ending(s)?;
    Ok((s, vd))
}

fn env_float(s: &str) -> IResult<&str, EnvType> {
    value(EnvType::Float, char('0')).parse(s)
}

fn env_int(s: &str) -> IResult<&str, EnvType> {
    value(EnvType::U64, char('1')).parse(s)
}

fn env_data(s: &str) -> IResult<&str, EnvType> {
    value(EnvType::U64, char('2')).parse(s)
}

fn env_var_type(s: &str) -> IResult<&str, EnvType> {
    alt((env_float, env_int, env_data)).parse(s)
}

fn dummy_node_vector_0(s: &str) -> IResult<&str, AccessType> {
    value(AccessType::DummyNodeVector0, char('0')).parse(s)
}

fn dummy_node_vector_1(s: &str) -> IResult<&str, AccessType> {
    value(AccessType::DummyNodeVector1, char('1')).parse(s)
}

fn dummy_node_vector_2(s: &str) -> IResult<&str, AccessType> {
    value(AccessType::DummyNodeVector2, char('2')).parse(s)
}
fn dummy_node_vector_3(s: &str) -> IResult<&str, AccessType> {
    value(AccessType::DummyNodeVector3, char('3')).parse(s)
}

fn access_type(s: &str) -> IResult<&str, AccessType> {
    let (s, _) = tag("DUMMY_NODE_VECTOR").parse(s)?;
    let (s, node) = alt((
        dummy_node_vector_0,
        dummy_node_vector_1,
        dummy_node_vector_2,
        dummy_node_vector_3,
    ))
    .parse(s)?;
    Ok((s, node))
}

fn access_node_vector_xxx(s: &str) -> IResult<&str, AccessNode> {
    value(AccessNode::VectorXXX, tag("VECTOR_XXX")).parse(s)
}

fn access_node_name(s: &str) -> IResult<&str, AccessNode> {
    map(c_ident, AccessNode::Name).parse(s)
}

fn access_node(s: &str) -> IResult<&str, AccessNode> {
    alt((access_node_vector_xxx, access_node_name)).parse(s)
}

/// Environment Variable Definitions
pub(crate) fn environment_variable(s: &str) -> IResult<&str, EnvironmentVariable> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("EV_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, env_var_name) = c_ident(s)?;
    let (s, _) = colon(s)?;
    let (s, _) = ms1(s)?;
    let (s, env_var_type) = env_var_type(s)?;
    let (s, _) = ms1(s)?;
    let (s, _) = brk_open(s)?;
    let (s, min) = complete::i64(s)?;
    let (s, _) = pipe(s)?;
    let (s, max) = complete::i64(s)?;
    let (s, _) = brk_close(s)?;
    let (s, _) = ms1(s)?;
    let (s, unit) = char_string(s)?;
    let (s, _) = ms1(s)?;
    let (s, initial_value) = double(s)?;
    let (s, _) = ms1(s)?;
    let (s, ev_id) = complete::i64(s)?;
    let (s, _) = ms1(s)?;
    let (s, access_type) = access_type(s)?;
    let (s, _) = ms1(s)?;
    let (s, access_nodes) = separated_list0(comma, access_node).parse(s)?;
    let (s, _) = semi_colon(s)?;
    let (s, _) = line_ending(s)?;
    Ok((
        s,
        EnvironmentVariable {
            name: env_var_name,
            typ: env_var_type,
            min,
            max,
            unit: unit.to_string(),
            initial_value,
            ev_id,
            access_type,
            access_nodes,
        },
    ))
}

pub(crate) fn environment_variable_data(s: &str) -> IResult<&str, EnvironmentVariableData> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("ENVVAR_DATA_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, env_var_name) = c_ident(s)?;
    let (s, _) = colon(s)?;
    let (s, _) = ms1(s)?;
    let (s, data_size) = complete::u64(s)?;
    let (s, _) = semi_colon(s)?;
    let (s, _) = line_ending(s)?;
    Ok((
        s,
        EnvironmentVariableData {
            env_var_name,
            data_size,
        },
    ))
}

pub(crate) fn signal_type(s: &str) -> IResult<&str, SignalType> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("SGTYPE_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, signal_type_name) = c_ident(s)?;
    let (s, _) = colon(s)?;
    let (s, _) = ms1(s)?;
    let (s, signal_size) = complete::u64(s)?;
    let (s, _) = at(s)?;
    let (s, byte_order) = byte_order(s)?;
    let (s, value_type) = value_type(s)?;
    let (s, _) = ms1(s)?;
    let (s, _) = brc_open(s)?;
    let (s, factor) = double(s)?;
    let (s, _) = comma(s)?;
    let (s, offset) = double(s)?;
    let (s, _) = brc_close(s)?;
    let (s, _) = ms1(s)?;
    let (s, _) = brk_open(s)?;
    let (s, min) = double(s)?;
    let (s, _) = pipe(s)?;
    let (s, max) = double(s)?;
    let (s, _) = brk_close(s)?;
    let (s, _) = ms1(s)?;
    let (s, unit) = char_string(s)?;
    let (s, _) = ms1(s)?;
    let (s, default_value) = double(s)?;
    let (s, _) = ms1(s)?;
    let (s, value_table) = c_ident(s)?;
    let (s, _) = semi_colon(s)?;
    let (s, _) = line_ending(s)?;
    Ok((
        s,
        SignalType {
            name: signal_type_name,
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

#[allow(dead_code)]
fn attribute_value_uint64(s: &str) -> IResult<&str, AttributeValue> {
    map(complete::u64, AttributeValue::U64).parse(s)
}

#[allow(dead_code)]
fn attribute_value_int64(s: &str) -> IResult<&str, AttributeValue> {
    map(complete::i64, AttributeValue::I64).parse(s)
}

fn attribute_value_f64(s: &str) -> IResult<&str, AttributeValue> {
    map(double, AttributeValue::Double).parse(s)
}

fn attribute_value_charstr(s: &str) -> IResult<&str, AttributeValue> {
    map(char_string, |x| AttributeValue::String(x.to_string())).parse(s)
}

pub(crate) fn attribute_value(s: &str) -> IResult<&str, AttributeValue> {
    alt((
        // attribute_value_uint64,
        // attribute_value_int64,
        attribute_value_f64,
        attribute_value_charstr,
    ))
    .parse(s)
}

fn network_node_attribute_value(s: &str) -> IResult<&str, AttributeValuedForObjectType> {
    let (s, _) = tag("BU_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, node_name) = c_ident(s)?;
    let (s, _) = ms1(s)?;
    let (s, value) = attribute_value(s)?;
    Ok((
        s,
        AttributeValuedForObjectType::NetworkNode(node_name, value),
    ))
}

fn message_definition_attribute_value(s: &str) -> IResult<&str, AttributeValuedForObjectType> {
    let (s, _) = tag("BO_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, message_id) = message_id(s)?;
    let (s, _) = ms1(s)?;
    let (s, value) = opt(attribute_value).parse(s)?;
    Ok((
        s,
        AttributeValuedForObjectType::MessageDefinition(message_id, value),
    ))
}

fn signal_attribute_value(s: &str) -> IResult<&str, AttributeValuedForObjectType> {
    let (s, _) = tag("SG_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, message_id) = message_id(s)?;
    let (s, _) = ms1(s)?;
    let (s, signal_name) = c_ident(s)?;
    let (s, _) = ms1(s)?;
    let (s, value) = attribute_value(s)?;
    Ok((
        s,
        AttributeValuedForObjectType::Signal(message_id, signal_name, value),
    ))
}

fn env_variable_attribute_value(s: &str) -> IResult<&str, AttributeValuedForObjectType> {
    let (s, _) = tag("EV_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, env_var_name) = c_ident(s)?;
    let (s, _) = ms1(s)?;
    let (s, value) = attribute_value(s)?;
    Ok((
        s,
        AttributeValuedForObjectType::EnvVariable(env_var_name, value),
    ))
}

fn raw_attribute_value(s: &str) -> IResult<&str, AttributeValuedForObjectType> {
    map(attribute_value, AttributeValuedForObjectType::Raw).parse(s)
}

pub(crate) fn attribute_value_for_object(s: &str) -> IResult<&str, AttributeValueForObject> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("BA_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, attribute_name) = char_string(s)?;
    let (s, _) = ms1(s)?;
    let (s, attribute_value) = alt((
        network_node_attribute_value,
        message_definition_attribute_value,
        signal_attribute_value,
        env_variable_attribute_value,
        raw_attribute_value,
    ))
    .parse(s)?;
    let (s, _) = semi_colon(s)?;
    let (s, _) = line_ending(s)?;
    Ok((
        s,
        AttributeValueForObject {
            name: attribute_name.to_string(),
            value: attribute_value,
        },
    ))
}

// TODO add properties
fn attribute_definition_node(s: &str) -> IResult<&str, AttributeDefinition> {
    let (s, _) = tag("BU_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, node) = take_till(is_semi_colon).parse(s)?;
    Ok((s, AttributeDefinition::Node(node.trim().to_string())))
}

// TODO add properties
fn attribute_definition_signal(s: &str) -> IResult<&str, AttributeDefinition> {
    let (s, _) = tag("SG_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, signal) = take_till(is_semi_colon).parse(s)?;
    Ok((s, AttributeDefinition::Signal(signal.trim().to_string())))
}

// TODO add properties
fn attribute_definition_environment_variable(s: &str) -> IResult<&str, AttributeDefinition> {
    let (s, _) = tag("EV_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, env_var) = take_till(is_semi_colon).parse(s)?;
    let value = env_var.trim().to_string();
    Ok((s, AttributeDefinition::EnvironmentVariable(value)))
}

// TODO add properties
fn attribute_definition_message(s: &str) -> IResult<&str, AttributeDefinition> {
    let (s, _) = tag("BO_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, message) = take_till(is_semi_colon).parse(s)?;
    Ok((s, AttributeDefinition::Message(message.trim().to_string())))
}

// TODO add properties
fn attribute_definition_plain(s: &str) -> IResult<&str, AttributeDefinition> {
    let (s, plain) = take_till(is_semi_colon).parse(s)?;
    Ok((s, AttributeDefinition::Plain(plain.trim().to_string())))
}

pub(crate) fn attribute_definition(s: &str) -> IResult<&str, AttributeDefinition> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("BA_DEF_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, def) = alt((
        attribute_definition_node,
        attribute_definition_signal,
        attribute_definition_environment_variable,
        attribute_definition_message,
        attribute_definition_plain,
    ))
    .parse(s)?;

    let (s, _) = semi_colon(s)?;
    let (s, _) = line_ending(s)?;
    Ok((s, def))
}

fn symbol(s: &str) -> IResult<&str, Symbol> {
    let (s, _) = space1(s)?;
    let (s, symbol) = c_ident(s)?;
    let (s, _) = line_ending(s)?;
    Ok((s, Symbol(symbol)))
}

pub(crate) fn new_symbols(s: &str) -> IResult<&str, Vec<Symbol>> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("NS_ :").parse(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = line_ending(s)?;
    let (s, symbols) = many0(symbol).parse(s)?;
    Ok((s, symbols))
}

/// Network node
pub(crate) fn node(s: &str) -> IResult<&str, Vec<Node>> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("BU_:").parse(s)?;
    let (s, li) = opt(preceded(ms1, separated_list0(ms1, c_ident))).parse(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = line_ending(s)?;
    Ok((
        s,
        li.map(|v| v.into_iter().map(Node).collect::<Vec<_>>())
            .unwrap_or_default(),
    ))
}

fn signal_type_ref(s: &str) -> IResult<&str, SignalTypeRef> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("SGTYPE_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, message_id) = message_id(s)?;
    let (s, _) = ms1(s)?;
    let (s, signal_name) = c_ident(s)?;
    let (s, _) = ms1(s)?;
    let (s, _) = colon(s)?;
    let (s, _) = ms1(s)?;
    let (s, signal_type_name) = c_ident(s)?;
    let (s, _) = semi_colon(s)?;
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

pub(crate) fn value_table(s: &str) -> IResult<&str, ValueTable> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("VAL_TABLE_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, value_table_name) = c_ident(s)?;
    let (s, value_descriptions) =
        many_till(preceded(ms0, value_description), preceded(ms0, semi_colon)).parse(s)?;
    let (s, _) = line_ending(s)?;
    Ok((
        s,
        ValueTable {
            name: value_table_name,
            descriptions: value_descriptions.0,
        },
    ))
}

fn extended_multiplex_mapping(s: &str) -> IResult<&str, ExtendedMultiplexMapping> {
    let (s, _) = ms0(s)?;
    let (s, min_value) = complete::u64(s)?;
    let (s, _) = char('-').parse(s)?;
    let (s, max_value) = complete::u64(s)?;
    Ok((
        s,
        ExtendedMultiplexMapping {
            min_value,
            max_value,
        },
    ))
}

pub(crate) fn extended_multiplex(s: &str) -> IResult<&str, ExtendedMultiplex> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("SG_MUL_VAL_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, message_id) = message_id(s)?;
    let (s, _) = ms1(s)?;
    let (s, signal_name) = c_ident(s)?;
    let (s, _) = ms1(s)?;
    let (s, multiplexor_signal_name) = c_ident(s)?;
    let (s, _) = ms1(s)?;
    let (s, mappings) = separated_list0(tag(","), extended_multiplex_mapping).parse(s)?;
    let (s, _) = semi_colon(s)?;
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

fn signed_or_unsigned_integer(s: &str) -> IResult<&str, SignalExtendedValueType> {
    value(SignalExtendedValueType::SignedOrUnsignedInteger, tag("0")).parse(s)
}
fn ieee_float_32bit(s: &str) -> IResult<&str, SignalExtendedValueType> {
    value(SignalExtendedValueType::IEEEfloat32Bit, tag("1")).parse(s)
}
fn ieee_double_64bit(s: &str) -> IResult<&str, SignalExtendedValueType> {
    value(SignalExtendedValueType::IEEEdouble64bit, tag("2")).parse(s)
}

fn signal_extended_value_type(s: &str) -> IResult<&str, SignalExtendedValueType> {
    alt((
        signed_or_unsigned_integer,
        ieee_float_32bit,
        ieee_double_64bit,
    ))
    .parse(s)
}

pub(crate) fn signal_extended_value_type_list(
    s: &str,
) -> IResult<&str, SignalExtendedValueTypeList> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("SIG_VALTYPE_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, message_id) = message_id(s)?;
    let (s, _) = ms1(s)?;
    let (s, signal_name) = c_ident(s)?;
    let (s, _) = ms1(s)?;
    let (s, _) = opt(colon).parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, signal_extended_value_type) = signal_extended_value_type(s)?;
    let (s, _) = semi_colon(s)?;
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

fn transmitter_vector_xxx(s: &str) -> IResult<&str, Transmitter> {
    value(Transmitter::VectorXXX, tag("Vector__XXX")).parse(s)
}

fn transmitter_node_name(s: &str) -> IResult<&str, Transmitter> {
    map(c_ident, Transmitter::NodeName).parse(s)
}

fn transmitter(s: &str) -> IResult<&str, Transmitter> {
    alt((transmitter_vector_xxx, transmitter_node_name)).parse(s)
}

fn message_transmitters(s: &str) -> IResult<&str, Vec<Transmitter>> {
    separated_list0(comma, transmitter).parse(s)
}

pub(crate) fn message_transmitter(s: &str) -> IResult<&str, MessageTransmitter> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("BO_TX_BU_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, message_id) = message_id(s)?;
    let (s, _) = ms1(s)?;
    let (s, _) = colon(s)?;
    let (s, _) = opt(ms1).parse(s)?;
    let (s, transmitter) = message_transmitters(s)?;
    let (s, _) = semi_colon(s)?;
    let (s, _) = line_ending(s)?;
    Ok((
        s,
        MessageTransmitter {
            message_id,
            transmitter,
        },
    ))
}

pub(crate) fn signal_groups(s: &str) -> IResult<&str, SignalGroups> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("SIG_GROUP_").parse(s)?;
    let (s, _) = ms1(s)?;
    let (s, message_id) = message_id(s)?;
    let (s, _) = ms1(s)?;
    let (s, signal_group_name) = c_ident(s)?;
    let (s, _) = ms1(s)?;
    let (s, repetitions) = complete::u64(s)?;
    let (s, _) = ms1(s)?;
    let (s, _) = colon(s)?;
    let (s, _) = ms1(s)?;
    let (s, signal_names) = separated_list0(ms1, c_ident).parse(s)?;
    let (s, _) = semi_colon(s)?;
    let (s, _) = line_ending(s)?;
    Ok((
        s,
        SignalGroups {
            message_id,
            name: signal_group_name,
            repetitions,
            signal_names,
        },
    ))
}

pub fn dbc(s: &str) -> IResult<&str, Dbc> {
    let (
        s,
        (
            version,
            new_symbols,
            bit_timing,
            nodes,
            value_tables,
            messages,
            message_transmitters,
            environment_variables,
            environment_variable_data,
            signal_types,
            comments,
            attribute_definitions,
            attribute_defaults,
            attribute_values,
            value_descriptions,
            signal_type_refs,
            signal_groups,
            signal_extended_value_type_list,
            extended_multiplex,
        ),
    ) = permutation((
        version,
        new_symbols,
        opt(bit_timing),
        many0(node),
        many0(value_table),
        many0(message),
        many0(message_transmitter),
        many0(environment_variable),
        many0(environment_variable_data),
        many0(signal_type),
        many0(comment),
        many0(attribute_definition),
        many0(attribute_default),
        many0(attribute_value_for_object),
        many0(value_descriptions),
        many0(signal_type_ref),
        many0(signal_groups),
        many0(signal_extended_value_type_list),
        many0(extended_multiplex),
    ))
    .parse(s)?;
    let (s, _) = multispace0(s)?;
    Ok((
        s,
        Dbc {
            version,
            new_symbols,
            bit_timing,
            nodes: nodes.into_iter().flatten().collect(),
            value_tables,
            messages,
            message_transmitters,
            environment_variables,
            environment_variable_data,
            signal_types,
            comments,
            attribute_definitions,
            attribute_defaults,
            attribute_values,
            value_descriptions,
            signal_type_refs,
            signal_groups,
            signal_extended_value_type_list,
            extended_multiplex,
        },
    ))
}
