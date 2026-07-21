//! Attribute value (`BA_`) parsing tests via the public `Dbc` API.
//!
//! `AttributeValueForObject` is crate-private; these tests assert the public
//! split attribute-value collections that `Dbc` builds from `BA_` lines.

use can_dbc::{
    AttributeValue, AttributeValueForDatabase, AttributeValueForEnvVariable,
    AttributeValueForMessage, AttributeValueForNode, AttributeValueForSignal, Dbc, MessageId,
};

fn parse_ba(ba_line: &str) -> Dbc {
    let src = format!(
        r#"VERSION ""
NS_ :
BS_:
BU_:
{ba_line}
"#
    );
    Dbc::try_from(src.as_str()).unwrap_or_else(|e| panic!("failed to parse BA_ snippet: {e}"))
}

#[test]
fn network_node_attribute_value() {
    let dbc = parse_ba(r#"BA_ "AttrName" BU_ NodeName 12;"#);
    assert_eq!(
        dbc.attribute_values_node,
        vec![AttributeValueForNode {
            name: "AttrName".to_string(),
            node_name: "NodeName".to_string(),
            value: AttributeValue::Uint(12),
        }]
    );
}

#[test]
fn integer_and_float_attributes() {
    let dbc = parse_ba(r#"BA_ "Attribute" BU_ NodeName 12;"#);
    assert_eq!(dbc.attribute_values_node[0].value, AttributeValue::Uint(12));

    let dbc = parse_ba(r#"BA_ "Attribute" BU_ NodeName -12;"#);
    assert_eq!(dbc.attribute_values_node[0].value, AttributeValue::Int(-12));

    let dbc = parse_ba(r#"BA_ "Attribute" BU_ NodeName 12.1;"#);
    assert_eq!(
        dbc.attribute_values_node[0].value,
        AttributeValue::Double(12.1)
    );
}

#[test]
fn message_definition_attribute_value() {
    let dbc = parse_ba(r#"BA_ "AttrName" BO_ 298 13;"#);
    assert_eq!(
        dbc.attribute_values_message,
        vec![AttributeValueForMessage {
            name: "AttrName".to_string(),
            message_id: MessageId::Standard(298),
            value: AttributeValue::Uint(13),
        }]
    );
}

#[test]
fn signal_attribute_value() {
    let dbc = parse_ba(r#"BA_ "AttrName" SG_ 198 SGName 13;"#);
    assert_eq!(
        dbc.attribute_values_signal,
        vec![AttributeValueForSignal {
            name: "AttrName".to_string(),
            message_id: MessageId::Standard(198),
            signal_name: "SGName".to_string(),
            value: AttributeValue::Uint(13),
        }]
    );
}

#[test]
fn env_var_attribute_value() {
    let dbc = parse_ba(r#"BA_ "AttrName" EV_ EvName "CharStr";"#);
    assert_eq!(
        dbc.attribute_values_env,
        vec![AttributeValueForEnvVariable {
            name: "AttrName".to_string(),
            variable_name: "EvName".to_string(),
            value: AttributeValue::String("CharStr".to_string()),
        }]
    );
}

#[test]
fn raw_attribute_value() {
    let dbc = parse_ba(r#"BA_ "AttrName" "RAW";"#);
    assert_eq!(
        dbc.attribute_values_database,
        vec![AttributeValueForDatabase {
            name: "AttrName".to_string(),
            value: AttributeValue::String("RAW".to_string()),
        }]
    );
}
