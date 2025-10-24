use can_dbc_pest::{Pair, Rule};

use crate::DbcResult;

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

/// Parse attribute definition: `BA_DEF_ [object_type] attribute_name attribute_type [min max];`
pub(crate) fn parse_attribute_definition(pair: Pair<Rule>) -> DbcResult<AttributeDefinition> {
    let mut definition_string = String::new();
    let mut object_type = None;

    // Collect all tokens to build the full definition string
    for pair2 in pair.into_inner() {
        match pair2.as_rule() {
            Rule::object_type => {
                // This is the new rule that captures the object type
                let text = pair2.as_str();
                if text == "SG_" {
                    object_type = Some("signal");
                } else if text == "BO_" {
                    object_type = Some("message");
                } else if text == "BU_" {
                    object_type = Some("node");
                } else if text == "EV_" {
                    object_type = Some("environment_variable");
                }
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
                definition_string.push_str(pair2.as_str());
            }
            other => panic!("What is this? {other:?}"),
        }
    }

    // Return appropriate attribute definition based on object type
    match object_type {
        Some("signal") => Ok(AttributeDefinition::Signal(definition_string)),
        Some("message") => Ok(AttributeDefinition::Message(definition_string)),
        Some("node") => Ok(AttributeDefinition::Node(definition_string)),
        Some("environment_variable") => {
            Ok(AttributeDefinition::EnvironmentVariable(definition_string))
        }
        _ => Ok(AttributeDefinition::Plain(definition_string)),
    }
}
