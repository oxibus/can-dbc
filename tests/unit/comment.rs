//! Unit tests for `comment`.

use can_dbc::{Comment, MessageId};
use can_dbc_pest::Rule;

use crate::common::test_into;

#[test]
fn signal_comment_test() {
    let def = r#"
CM_ SG_ 193 KLU_R_X "This is a signal comment test";
"#;
    let exp = Comment::Signal {
        message_id: MessageId::Standard(193),
        name: "KLU_R_X".to_string(),
        comment: "This is a signal comment test".to_string(),
    };
    let val = test_into::<Comment>(def.trim_start(), Rule::comment);
    assert_eq!(val, exp);
}

#[test]
fn message_definition_comment_test() {
    let def = r#"
CM_ BO_ 34544 "Some Message comment";
"#;
    let exp = Comment::Message {
        id: MessageId::Standard(34544),
        comment: "Some Message comment".to_string(),
    };
    let val = test_into::<Comment>(def.trim_start(), Rule::comment);
    assert_eq!(val, exp);
}

#[test]
fn node_comment_test() {
    let def = r#"
CM_ BU_ network_node "Some network node comment";
"#;
    let exp = Comment::Node {
        name: "network_node".to_string(),
        comment: "Some network node comment".to_string(),
    };
    let val = test_into::<Comment>(def.trim_start(), Rule::comment);
    assert_eq!(val, exp);
}

#[test]
fn env_var_comment_test() {
    let def = r#"
CM_ EV_ ENVXYZ "Some env var name comment";
"#;
    let exp = Comment::EnvVar {
        name: "ENVXYZ".to_string(),
        comment: "Some env var name comment".to_string(),
    };
    let val = test_into::<Comment>(def.trim_start(), Rule::comment);
    assert_eq!(val, exp);
}

#[test]
fn signal_comment_with_escaped_characters_test() {
    let def = r#"
CM_ SG_ 2147548912 FooBar "Foo\\ \n \"Bar\"";
"#;
    let exp = Comment::Signal {
        message_id: MessageId::Extended(65264),
        name: "FooBar".to_string(),
        comment: r#"Foo\\ \n \"Bar\""#.to_string(),
    };
    let val = test_into::<Comment>(def.trim_start(), Rule::comment);
    assert_eq!(val, exp);
}

#[test]
fn empty_signal_comment_test() {
    let def = r#"
CM_ SG_ 2147548912 FooBar "";
"#;
    let exp = Comment::Signal {
        message_id: MessageId::Extended(65264),
        name: "FooBar".to_string(),
        comment: String::new(),
    };
    let val = test_into::<Comment>(def.trim_start(), Rule::comment);
    assert_eq!(val, exp);
}
