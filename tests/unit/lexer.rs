//! Lexical helper tests (idents, strings, numbers).

use can_dbc::Symbol;
use can_dbc_pest::Rule;

use crate::common::{parse, span};

#[test]
fn c_ident() {
    assert_eq!(span("EALL_DUSasb18 ", Rule::ident), "EALL_DUSasb18");
    assert_eq!(span("_EALL_DUSasb18 ", Rule::ident), "_EALL_DUSasb18");
    assert!(parse("3EALL_DUSasb18 ", Rule::ident).is_err());
}

#[test]
fn c_ident_to_symbol() {
    let def = "FZHL_DUSasb18 ";
    let val = vec![Symbol(span(def, Rule::ident).to_string())];
    assert_eq!(val, vec![Symbol("FZHL_DUSasb18".to_string())]);

    let def = "FZHL_DUSasb19,xkask_3298 ";
    let val = vec![Symbol(span(def, Rule::ident).to_string())];
    assert_eq!(val, vec![Symbol("FZHL_DUSasb19".to_string())]);
}

#[test]
fn char_string() {
    let def = "\"ab\x00\x7f\"";
    assert_eq!(span(def, Rule::quoted_str), "\"ab\x00\x7f\"");
}

#[test]
fn attribute_value_f64() {
    let val = span("80.0", Rule::number)
        .parse::<f64>()
        .expect("number should parse as f64");
    assert!((val - 80.0).abs() < f64::EPSILON);
}
