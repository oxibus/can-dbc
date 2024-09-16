#[cfg(feature = "with-serde")]
extern crate serde;
#[cfg(feature = "with-serde")]
#[macro_use]
extern crate serde_derive;

use derive_getters::Getters;

use crate::DBCString;
use crate::message::MessageId;

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
}
