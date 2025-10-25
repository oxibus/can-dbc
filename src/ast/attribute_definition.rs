use can_dbc_pest::{Pair, Rule};

use crate::parser::DbcResult;

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

impl AttributeDefinition {
    /// Parse attribute definition: `BA_DEF_ [object_type] attribute_name attribute_type [min max];`
    pub(crate) fn parse(pair: Pair<Rule>) -> DbcResult<AttributeDefinition> {
        let mut definition_string = String::new();
        let mut object_type = "";

        for pairs in pair.into_inner() {
            match pairs.as_rule() {
                Rule::object_type => object_type = pairs.as_str(),
                Rule::attribute_name
                | Rule::attribute_type_int
                | Rule::attribute_type_hex
                | Rule::attribute_type_float
                | Rule::attribute_type_string
                | Rule::attribute_type_enum => {
                    if !definition_string.is_empty() {
                        definition_string.push(' ');
                    }
                    definition_string.push_str(pairs.as_str());
                }
                _ => panic!("Unexpected rule: {:?}", pairs.as_rule()),
            }
        }

        Ok(match object_type {
            "SG_" => AttributeDefinition::Signal(definition_string),
            "BO_" => AttributeDefinition::Message(definition_string),
            "BU_" => AttributeDefinition::Node(definition_string),
            "EV_" => AttributeDefinition::EnvironmentVariable(definition_string),
            _ => AttributeDefinition::Plain(definition_string),
        })
    }
}
