//! Unit tests for `attribute_definition`.

use can_dbc::{AttributeDefinition, AttributeValueType, NumericValue};
use can_dbc_pest::Rule;

use crate::common::test_into;

#[test]
fn attribute_definition_bo_test() {
    let def = r#"
BA_DEF_ BO_ "BaDef1BO" INT 0 1000000;
"#;
    let exp = AttributeDefinition::Message(
        "BaDef1BO".to_string(),
        AttributeValueType::Int(NumericValue::Uint(0), NumericValue::Uint(1_000_000)),
    );
    let val = test_into::<AttributeDefinition>(def.trim_start(), Rule::attr_def);
    assert_eq!(val, exp);
}

#[test]
fn attribute_definition_bo_txt_test() {
    let def = r#"
BA_DEF_ BO_  "GenMsgSendType" STRING ;
"#;
    let exp =
        AttributeDefinition::Message("GenMsgSendType".to_string(), AttributeValueType::String);
    let val = test_into::<AttributeDefinition>(def.trim_start(), Rule::attr_def);
    assert_eq!(val, exp);
}

#[test]
fn attribute_definition_bu_test() {
    let def = r#"
BA_DEF_ BU_ "BuDef1BO" INT 0 1000000;
"#;
    let exp = AttributeDefinition::Node(
        "BuDef1BO".to_string(),
        AttributeValueType::Int(NumericValue::Uint(0), NumericValue::Uint(1_000_000)),
    );
    let val = test_into::<AttributeDefinition>(def.trim_start(), Rule::attr_def);
    assert_eq!(val, exp);
}

#[test]
fn attribute_definition_sg_test() {
    let def = r#"
BA_DEF_ SG_ "SgDef1BO" INT 0 1000000;
"#;
    let exp = AttributeDefinition::Signal(
        "SgDef1BO".to_string(),
        AttributeValueType::Int(NumericValue::Uint(0), NumericValue::Uint(1_000_000)),
    );
    let val = test_into::<AttributeDefinition>(def.trim_start(), Rule::attr_def);
    assert_eq!(val, exp);
}

#[test]
fn attribute_definition_ev_test() {
    let def = r#"
BA_DEF_ EV_ "EvDef1BO" INT 0 1000000;
"#;
    let exp = AttributeDefinition::EnvironmentVariable(
        "EvDef1BO".to_string(),
        AttributeValueType::Int(NumericValue::Uint(0), NumericValue::Uint(1_000_000)),
    );
    let val = test_into::<AttributeDefinition>(def.trim_start(), Rule::attr_def);
    assert_eq!(val, exp);
}
