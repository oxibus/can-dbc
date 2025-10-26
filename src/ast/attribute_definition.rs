use can_dbc_pest::{Pair, Rule};

use crate::parser::DbcError;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

impl TryFrom<Pair<'_, Rule>> for AttributeDefinition {
    type Error = DbcError;

    /// Parse attribute definition: `BA_DEF_ [object_type] attribute_name attribute_type [min max];`
    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let pairs = pair.into_inner();
        let mut definition_string = String::new();
        let mut object_type = "";

        // Process all pairs
        for pair in pairs {
            match pair.as_rule() {
                Rule::object_type => {
                    object_type = pair.as_str();
                }
                Rule::attribute_name
                | Rule::attribute_type_int
                | Rule::attribute_type_hex
                | Rule::attribute_type_float
                | Rule::attribute_type_string
                | Rule::attribute_type_enum => {
                    if !definition_string.is_empty() {
                        definition_string.push(' ');
                    }
                    definition_string.push_str(pair.as_str());
                }
                _ => return Err(DbcError::ParseError),
            }
        }

        Ok(match object_type {
            "SG_" => Self::Signal(definition_string),
            "BO_" => Self::Message(definition_string),
            "BU_" => Self::Node(definition_string),
            "EV_" => Self::EnvironmentVariable(definition_string),
            _ => Self::Plain(definition_string),
        })
    }
}
