#[cfg(feature = "with-serde")]
extern crate serde;
#[cfg(feature = "with-serde")]
#[macro_use]
extern crate serde_derive;

use derive_getters::Getters;

use crate::DBCString;
use crate::message::MessageId;

use crate::parser;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    multi::{many_till},
    sequence::preceded,
    combinator::{map, opt},
    character::complete::{self, line_ending, multispace0},
    number::complete::double,
    IResult,
};

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct AttributeDefault {
    pub (crate) attribute_name: String,
    pub (crate) attribute_value: AttributeValue,
}

impl DBCString for AttributeDefault {
    fn dbc_string(&self) -> String {
        return format!("BA_DEF_DEF_ \"{}\" {};",
            self.attribute_name,
            self.attribute_value.dbc_string(),
        )
    }
    
    fn parse(s: &str) -> IResult<&str, Self>
        where
            Self: Sized {
        let (s, _) = multispace0(s)?;
        let (s, _) = tag("BA_DEF_DEF_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, attribute_name) = parser::char_string(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, attribute_value) = AttributeValue::parse(s)?;
        let (s, _) = parser::semi_colon(s)?;
        let (s, _) = line_ending(s)?;

        Ok((
            s,
            AttributeDefault {
                attribute_name: attribute_name.to_string(),
                attribute_value,
            },
        ))
    }
}

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct AttributeValueForObject {
    pub (crate) attribute_name: String,
    pub (crate) attribute_value: AttributeValuedForObjectType,
}

impl DBCString for AttributeValueForObject {
    fn dbc_string(&self) -> String {
        return format!("BA_ \"{}\" {};",
            self.attribute_name,
            self.attribute_value.dbc_string(),
        )
    }

    fn parse(s: &str) -> IResult<&str, Self>
        where
            Self: Sized {
        let (s, _) = multispace0(s)?;
        let (s, _) = tag("BA_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, attribute_name) = parser::char_string(s)?;
        let (s, _) = parser::ms1(s)?;
        // let (s, attribute_value) = alt((
        //     network_node_attribute_value,
        //     message_definition_attribute_value,
        //     signal_attribute_value,
        //     env_variable_attribute_value,
        //     raw_attribute_value,
        // ))(s)?;
        let (s, attribute_value) = AttributeValuedForObjectType::parse(s)?;
        let (s, _) = parser::semi_colon(s)?;
        let (s, _) = line_ending(s)?;
        Ok((
            s,
            AttributeValueForObject {
                attribute_name: attribute_name.to_string(),
                attribute_value,
            },
        ))
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

impl AttributeDefinition {
    // TODO add properties
    fn attribute_definition_node(s: &str) -> IResult<&str, AttributeDefinition> {
        let (s, _) = tag("BU_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, node) = take_till(parser::is_semi_colon)(s)?;
        Ok((s, AttributeDefinition::Node(node.to_string())))
    }

    // TODO add properties
    fn attribute_definition_signal(s: &str) -> IResult<&str, AttributeDefinition> {
        let (s, _) = tag("SG_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, signal) = take_till(parser::is_semi_colon)(s)?;
        Ok((s, AttributeDefinition::Signal(signal.to_string())))
    }

    // TODO add properties
    fn attribute_definition_environment_variable(s: &str) -> IResult<&str, AttributeDefinition> {
        let (s, _) = tag("EV_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, env_var) = take_till(parser::is_semi_colon)(s)?;
        Ok((
            s,
            AttributeDefinition::EnvironmentVariable(env_var.to_string()),
        ))
    }

    // TODO add properties
    fn attribute_definition_message(s: &str) -> IResult<&str, AttributeDefinition> {
        let (s, _) = tag("BO_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, message) = take_till(parser::is_semi_colon)(s)?;
        Ok((s, AttributeDefinition::Message(message.to_string())))
    }

    // TODO add properties
    fn attribute_definition_plain(s: &str) -> IResult<&str, AttributeDefinition> {
        let (s, plain) = take_till(parser::is_semi_colon)(s)?;
        Ok((s, AttributeDefinition::Plain(plain.to_string())))
    }
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

    fn parse(s: &str) -> IResult<&str, Self>
        where
            Self: Sized {
        let (s, _) = multispace0(s)?;
        let (s, _) = tag("BA_DEF_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, def) = alt((
            Self::attribute_definition_node,
            Self::attribute_definition_signal,
            Self::attribute_definition_environment_variable,
            Self::attribute_definition_message,
            Self::attribute_definition_plain,
        ))(s)?;
    
        let (s, _) = parser::semi_colon(s)?;
        let (s, _) = line_ending(s)?;
        Ok((s, def))
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

impl ValueDescription {
    fn value_description_for_signal(s: &str) -> IResult<&str, ValueDescription> {
        let (s, _) = parser::ms0(s)?;
        let (s, _) = tag("VAL_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, message_id) = MessageId::parse(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, signal_name) = parser::c_ident(s)?;
        let (s, value_descriptions) = many_till(
            preceded(parser::ms1, ValDescription::parse),
            preceded(opt(parser::ms1), parser::semi_colon),
        )(s)?;
        Ok((
            s,
            ValueDescription::Signal {
                message_id,
                signal_name,
                value_descriptions: value_descriptions.0,
            },
        ))
    }

    fn value_description_for_env_var(s: &str) -> IResult<&str, ValueDescription> {
        let (s, _) = parser::ms0(s)?;
        let (s, _) = tag("VAL_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, env_var_name) = parser::c_ident(s)?;
        let (s, value_descriptions) = many_till(
            preceded(parser::ms1, ValDescription::parse),
            preceded(opt(parser::ms1), parser::semi_colon),
        )(s)?;
        Ok((
            s,
            ValueDescription::EnvironmentVariable {
                env_var_name,
                value_descriptions: value_descriptions.0,
            },
        ))
    }
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

    fn parse(s: &str) -> IResult<&str, Self>
        where
            Self: Sized {
        let (s, _) = multispace0(s)?;
        let (s, vd) = alt((Self::value_description_for_signal, Self::value_description_for_env_var))(s)?;
        let (s, _) = line_ending(s)?;
        Ok((s, vd))
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

impl AttributeValuedForObjectType {
    fn network_node_attribute_value(s: &str) -> IResult<&str, AttributeValuedForObjectType> {
        let (s, _) = tag("BU_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, node_name) = parser::c_ident(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, value) = AttributeValue::parse(s)?;
        Ok((
            s,
            AttributeValuedForObjectType::NetworkNodeAttributeValue(node_name, value),
        ))
    }

    fn message_definition_attribute_value(s: &str) -> IResult<&str, AttributeValuedForObjectType> {
        let (s, _) = tag("BO_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, message_id) = MessageId::parse(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, value) = opt(AttributeValue::parse)(s)?;
        Ok((
            s,
            AttributeValuedForObjectType::MessageDefinitionAttributeValue(message_id, value),
        ))
    }

    fn signal_attribute_value(s: &str) -> IResult<&str, AttributeValuedForObjectType> {
        let (s, _) = tag("SG_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, message_id) = MessageId::parse(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, signal_name) = parser::c_ident(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, value) = AttributeValue::parse(s)?;
        Ok((
            s,
            AttributeValuedForObjectType::SignalAttributeValue(message_id, signal_name, value),
        ))
    }

    fn env_variable_attribute_value(s: &str) -> IResult<&str, AttributeValuedForObjectType> {
        let (s, _) = tag("EV_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, env_var_name) = parser::c_ident(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, value) = AttributeValue::parse(s)?;
        Ok((
            s,
            AttributeValuedForObjectType::EnvVariableAttributeValue(env_var_name, value),
        ))
    }

    fn raw_attribute_value(s: &str) -> IResult<&str, AttributeValuedForObjectType> {
        map(
            AttributeValue::parse,
            AttributeValuedForObjectType::RawAttributeValue,
        )(s)
    }
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

    fn parse(s: &str) -> IResult<&str, Self>
        where
        Self: Sized {

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

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Debug, PartialEq)]
pub enum AttributeValue {
    AttributeValueU64(u64),
    AttributeValueI64(i64),
    AttributeValueF64(f64),
    AttributeValueCharString(String),
}

impl AttributeValue {
    #[allow(dead_code)]
    fn attribute_value_uint64(s: &str) -> IResult<&str, AttributeValue> {
        map(complete::u64, AttributeValue::AttributeValueU64)(s)
    }

    #[allow(dead_code)]
    fn attribute_value_int64(s: &str) -> IResult<&str, AttributeValue> {
        map(complete::i64, AttributeValue::AttributeValueI64)(s)
    }

    fn attribute_value_f64(s: &str) -> IResult<&str, AttributeValue> {
        map(double, AttributeValue::AttributeValueF64)(s)
    }

    fn attribute_value_charstr(s: &str) -> IResult<&str, AttributeValue> {
        map(parser::char_string, |x| {
            AttributeValue::AttributeValueCharString(x.to_string())
        })(s)
    }
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

    fn parse(s: &str) -> IResult<&str, Self>
        where
            Self: Sized {
        alt((
            // Self::attribute_value_uint64,
            // Self::attribute_value_int64,
            Self::attribute_value_f64,
            Self::attribute_value_charstr,
        ))(s)
    }
}

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct AttrDefault {
    name: String,
    value: AttributeValue,
}

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ValDescription {
    pub (crate) a: f64,
    pub (crate) b: String,
}

impl DBCString for ValDescription {
    fn dbc_string(&self) -> String {
        return format!("{} \"{}\"",
            self.a, self.b
        )
    }

    fn parse(s: &str) -> nom::IResult<&str, Self>
        where
            Self: Sized {
        let (s, a) = double(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, b) = parser::char_string(s)?;
        Ok((
            s,
            ValDescription {
                a,
                b: b.to_string(),
            },
        ))
    }
}

/// Global value table
#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ValueTable {
    pub (crate) value_table_name: String,
    pub (crate) value_descriptions: Vec<ValDescription>,
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

    fn parse(s: &str) -> nom::IResult<&str, Self>
        where
            Self: Sized {
        let (s, _) = multispace0(s)?;
        let (s, _) = tag("VAL_TABLE_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, value_table_name) = parser::c_ident(s)?;
        let (s, value_descriptions) =
            many_till(preceded(parser::ms0, ValDescription::parse), preceded(parser::ms0, parser::semi_colon))(s)?;
        let (s, _) = line_ending(s)?;
        Ok((
            s,
            ValueTable {
                value_table_name,
                value_descriptions: value_descriptions.0,
            },
        ))
    }
}
