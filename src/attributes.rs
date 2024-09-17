#[cfg(feature = "with-serde")]
extern crate serde;
#[cfg(feature = "with-serde")]
#[macro_use]
extern crate serde_derive;

use derive_getters::Getters;

use crate::MessageId;
use crate::DBCString;
use crate::parser;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{self, line_ending, multispace0},
    combinator::{map, opt},
    multi::many_till,
    number::complete::double,
    sequence::preceded,
    IResult,
};

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct AttributeDefault {
    pub(crate) attribute_name: String,
    pub(crate) attribute_value: AttributeValue,
}

impl DBCString for AttributeDefault {
    fn dbc_string(&self) -> String {
        return format!(
            "BA_DEF_DEF_ \"{}\" {};",
            self.attribute_name,
            self.attribute_value.dbc_string(),
        );
    }

    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
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

#[test]
fn attribute_default_test() {
    let def = "BA_DEF_DEF_  \"ZUV\" \"OAL\";\n";
    let (_, attr_default) = AttributeDefault::parse(def).unwrap();
    let attr_default_exp = AttributeDefault {
        attribute_name: "ZUV".to_string(),
        attribute_value: AttributeValue::AttributeValueCharString("OAL".to_string()),
    };
    assert_eq!(attr_default_exp, attr_default);
}

#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct AttributeValueForObject {
    pub(crate) attribute_name: String,
    pub(crate) attribute_value: AttributeValuedForObjectType,
}

impl DBCString for AttributeValueForObject {
    fn dbc_string(&self) -> String {
        return format!(
            "BA_ \"{}\" {};",
            self.attribute_name,
            self.attribute_value.dbc_string(),
        );
    }

    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        let (s, _) = multispace0(s)?;
        let (s, _) = tag("BA_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, attribute_name) = parser::char_string(s)?;
        let (s, _) = parser::ms1(s)?;
        // TODO delete this section if calling the enum parse works just as well
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

#[test]
fn network_node_attribute_value_test() {
    let def = "BA_ \"AttrName\" BU_ NodeName 12;\n";
    let attribute_value = AttributeValuedForObjectType::NetworkNodeAttributeValue(
        "NodeName".to_string(),
        AttributeValue::AttributeValueF64(12.0),
    );
    let attr_val_exp = AttributeValueForObject {
        attribute_name: "AttrName".to_string(),
        attribute_value,
    };
    let (_, attr_val) = AttributeValueForObject::parse(def).unwrap();
    assert_eq!(attr_val_exp, attr_val);
}

#[test]
fn message_definition_attribute_value_test() {
    let def = "BA_ \"AttrName\" BO_ 298 13;\n";
    let attribute_value = AttributeValuedForObjectType::MessageDefinitionAttributeValue(
        MessageId::Standard(298),
        Some(AttributeValue::AttributeValueF64(13.0)),
    );
    let attr_val_exp = AttributeValueForObject {
        attribute_name: "AttrName".to_string(),
        attribute_value,
    };
    let (_, attr_val) = AttributeValueForObject::parse(def).unwrap();
    assert_eq!(attr_val_exp, attr_val);
}

#[test]
fn signal_attribute_value_test() {
    let def = "BA_ \"AttrName\" SG_ 198 SGName 13;\n";
    let attribute_value = AttributeValuedForObjectType::SignalAttributeValue(
        MessageId::Standard(198),
        "SGName".to_string(),
        AttributeValue::AttributeValueF64(13.0),
    );
    let attr_val_exp = AttributeValueForObject {
        attribute_name: "AttrName".to_string(),
        attribute_value,
    };
    let (_, attr_val) = AttributeValueForObject::parse(def).unwrap();
    assert_eq!(attr_val_exp, attr_val);
}

#[test]
fn env_var_attribute_value_test() {
    let def = "BA_ \"AttrName\" EV_ EvName \"CharStr\";\n";
    let attribute_value = AttributeValuedForObjectType::EnvVariableAttributeValue(
        "EvName".to_string(),
        AttributeValue::AttributeValueCharString("CharStr".to_string()),
    );
    let attr_val_exp = AttributeValueForObject {
        attribute_name: "AttrName".to_string(),
        attribute_value,
    };
    let (_, attr_val) = AttributeValueForObject::parse(def).unwrap();
    assert_eq!(attr_val_exp, attr_val);
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
            Self::Plain(s) => format!("{};", s),
        };
    }

    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
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

#[test]
fn attribute_definition_test() {
    let def_bo = "BA_DEF_ BO_ \"BaDef1BO\" INT 0 1000000;\n";
    let (_, bo_def) = AttributeDefinition::parse(def_bo).unwrap();
    let bo_def_exp = AttributeDefinition::Message("\"BaDef1BO\" INT 0 1000000".to_string());
    assert_eq!(bo_def_exp, bo_def);

    let def_bu = "BA_DEF_ BU_ \"BuDef1BO\" INT 0 1000000;\n";
    let (_, bu_def) = AttributeDefinition::parse(def_bu).unwrap();
    let bu_def_exp = AttributeDefinition::Node("\"BuDef1BO\" INT 0 1000000".to_string());
    assert_eq!(bu_def_exp, bu_def);

    let def_signal = "BA_DEF_ SG_ \"SgDef1BO\" INT 0 1000000;\n";
    let (_, signal_def) = AttributeDefinition::parse(def_signal).unwrap();
    let signal_def_exp = AttributeDefinition::Signal("\"SgDef1BO\" INT 0 1000000".to_string());
    assert_eq!(signal_def_exp, signal_def);

    let def_env_var = "BA_DEF_ EV_ \"EvDef1BO\" INT 0 1000000;\n";
    let (_, env_var_def) = AttributeDefinition::parse(def_env_var).unwrap();
    let env_var_def_exp =
        AttributeDefinition::EnvironmentVariable("\"EvDef1BO\" INT 0 1000000".to_string());
    assert_eq!(env_var_def_exp, env_var_def);
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
            Self::Signal {
                message_id,
                signal_name,
                value_descriptions,
            } => {
                format!(
                    "VAL_ {} {} \"{}\";",
                    message_id.dbc_string(),
                    signal_name,
                    value_descriptions
                        .clone()
                        .into_iter()
                        .map(|vd| vd.dbc_string())
                        .collect::<Vec<String>>()
                        .join(";") // TODO verify this is the correct delimiter
                )
            }
            Self::EnvironmentVariable {
                env_var_name,
                value_descriptions,
            } => {
                format!(
                    "VAL_ {} \"{}\";",
                    env_var_name,
                    value_descriptions
                        .clone()
                        .into_iter()
                        .map(|vd| vd.dbc_string())
                        .collect::<Vec<String>>()
                        .join(";") // TODO verify this is the correct delimiter
                )
            }
        };
    }

    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        let (s, _) = multispace0(s)?;
        let (s, vd) = alt((
            Self::value_description_for_signal,
            Self::value_description_for_env_var,
        ))(s)?;
        let (s, _) = line_ending(s)?;
        Ok((s, vd))
    }
}

#[test]
fn value_description_for_signal_test() {
    let def1 = "VAL_ 837 UF_HZ_OI 255 \"NOP\";\n";
    let message_id = MessageId::Standard(837);
    let signal_name = "UF_HZ_OI".to_string();
    let val_descriptions = vec![ValDescription {
        a: 255.0,
        b: "NOP".to_string(),
    }];
    let value_description_for_signal1 = ValueDescription::Signal {
        message_id,
        signal_name,
        value_descriptions: val_descriptions,
    };
    let (_, value_signal_def) =
        ValueDescription::parse(def1).expect("Failed to parse value desc for signal");
    assert_eq!(value_description_for_signal1, value_signal_def);
}

#[test]
fn value_description_for_env_var_test() {
    let def1 = "VAL_ MY_ENV_VAR 255 \"NOP\";\n";
    let env_var_name = "MY_ENV_VAR".to_string();
    let val_descriptions = vec![ValDescription {
        a: 255.0,
        b: "NOP".to_string(),
    }];
    let value_env_var1 = ValueDescription::EnvironmentVariable {
        env_var_name,
        value_descriptions: val_descriptions,
    };
    let (_, value_env_var) =
        ValueDescription::parse(def1).expect("Failed to parse value desc for env var");
    assert_eq!(value_env_var1, value_env_var);
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
            }
            Self::MessageDefinitionAttributeValue(m_id, av) => {
                format!(
                    "BO_ {}{}",
                    m_id.dbc_string(),
                    match av {
                        None => "".to_string(),
                        Some(v) => format!(" {}", v.dbc_string()),
                    }
                )
            }
            Self::SignalAttributeValue(m_id, s, av) => {
                format!("SG_ {} {} {}", m_id.dbc_string(), s, av.dbc_string())
            }
            Self::EnvVariableAttributeValue(s, av) => {
                format!("EV_ {} {}", s, av.dbc_string())
            }
        };
    }

    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        alt((
            Self::network_node_attribute_value,
            Self::message_definition_attribute_value,
            Self::signal_attribute_value,
            Self::env_variable_attribute_value,
            Self::raw_attribute_value,
        ))(s)
    }
}

#[test]
fn raw_attribute_value_test() {
    let def = "BA_ \"AttrName\" \"RAW\";\n";
    let attribute_value = AttributeValuedForObjectType::RawAttributeValue(
        AttributeValue::AttributeValueCharString("RAW".to_string()),
    );
    let attr_val_exp = AttributeValueForObject {
        attribute_name: "AttrName".to_string(),
        attribute_value,
    };
    let (_, attr_val) = AttributeValueForObject::parse(def).unwrap();
    assert_eq!(attr_val_exp, attr_val);
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
        };
    }

    fn parse(s: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        alt((
            // Self::attribute_value_uint64,
            // Self::attribute_value_int64,
            Self::attribute_value_f64,
            Self::attribute_value_charstr,
        ))(s)
    }
}

#[test]
fn attribute_value_f64_test() {
    let def = "80.0";
    let (_, val) = AttributeValue::parse(def).unwrap();
    assert_eq!(AttributeValue::AttributeValueF64(80.0), val);
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
    pub(crate) a: f64,
    pub(crate) b: String,
}

impl DBCString for ValDescription {
    fn dbc_string(&self) -> String {
        return format!("{} \"{}\"", self.a, self.b);
    }

    fn parse(s: &str) -> nom::IResult<&str, Self>
    where
        Self: Sized,
    {
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

#[test]
fn value_description_test() {
    let def = "2 \"ABC\"\n";
    let exp = ValDescription {
        a: 2f64,
        b: "ABC".to_string(),
    };
    let (_, val_desc) = ValDescription::parse(def).unwrap();
    assert_eq!(exp, val_desc);
}

/// Global value table
#[derive(Clone, Debug, PartialEq, Getters)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ValueTable {
    pub(crate) value_table_name: String,
    pub(crate) value_descriptions: Vec<ValDescription>,
}

impl DBCString for ValueTable {
    fn dbc_string(&self) -> String {
        return format!(
            "VAL_TABLE_ {} {}",
            self.value_table_name,
            self.value_descriptions
                .clone()
                .into_iter()
                .map(|vd| vd.dbc_string())
                .collect::<Vec<String>>()
                .join(";")
        );
    }

    fn parse(s: &str) -> nom::IResult<&str, Self>
    where
        Self: Sized,
    {
        let (s, _) = multispace0(s)?;
        let (s, _) = tag("VAL_TABLE_")(s)?;
        let (s, _) = parser::ms1(s)?;
        let (s, value_table_name) = parser::c_ident(s)?;
        let (s, value_descriptions) = many_till(
            preceded(parser::ms0, ValDescription::parse),
            preceded(parser::ms0, parser::semi_colon),
        )(s)?;
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

#[test]
fn val_table_test() {
    let def = "VAL_TABLE_ Tst 2 \"ABC\" 1 \"Test A\" ;\n";
    let exp = ValueTable {
        value_table_name: "Tst".to_string(),
        value_descriptions: vec![
            ValDescription {
                a: 2f64,
                b: "ABC".to_string(),
            },
            ValDescription {
                a: 1f64,
                b: "Test A".to_string(),
            },
        ],
    };
    let (_, val_table) = ValueTable::parse(def).unwrap();
    assert_eq!(exp, val_table);
}

#[test]
fn val_table_no_space_preceding_comma_test() {
    let def = "VAL_TABLE_ Tst 2 \"ABC\";\n";
    let exp = ValueTable {
        value_table_name: "Tst".to_string(),
        value_descriptions: vec![ValDescription {
            a: 2f64,
            b: "ABC".to_string(),
        }],
    };
    let (_, val_table) = ValueTable::parse(def).unwrap();
    assert_eq!(exp, val_table);
}
