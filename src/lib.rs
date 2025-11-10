#![doc = include_str!("../README.md")]

mod ast;
mod extend;
mod parser;

// Re-export all types from the ast module
pub use ast::*;
// Re-export of `encoding_rs` as encodings to simplify usage
#[cfg(feature = "encodings")]
pub use encoding_rs as encodings;
pub use extend::*;
#[cfg(feature = "encodings")]
pub use parser::decode_cp1252;
pub use parser::{DbcError, DbcResult};

#[cfg(test)]
mod test_helpers {
    use std::fmt::Debug;

    use can_dbc_pest::*;

    use crate::parser::*;

    /// Helper function to parse a snippet with a specific rule, returning just the rule's inner content already unwrapped
    pub fn parse(input: &str, rule: Rule) -> DbcResult<Pair<'_, Rule>> {
        let pairs = DbcParser::parse(rule, input)?;
        Ok(pairs.into_iter().next().unwrap())
    }

    pub fn span(input: &str, rule: Rule) -> &str {
        let pair = parse(input, rule).unwrap();
        pair.as_span().as_str()
    }

    pub fn test_into<'a, T>(input: &'a str, rule: Rule) -> T
    where
        T: TryFrom<Pair<'a, Rule>>,
        <T as TryFrom<Pair<'a, Rule>>>::Error: Debug,
    {
        let pair = parse(input, rule).unwrap_or_else(|e| {
            panic!("Parse {e:?}:\n{input:#?}");
        });
        pair.clone().try_into().unwrap_or_else(|e| {
            panic!("Into {e:?}:\n{pair:#?}");
        })
    }

    #[test]
    fn c_ident_test() {
        assert_eq!(span("EALL_DUSasb18 ", Rule::ident), "EALL_DUSasb18");
        assert_eq!(span("_EALL_DUSasb18 ", Rule::ident), "_EALL_DUSasb18");
        assert!(parse("3EALL_DUSasb18 ", Rule::ident).is_err());
    }

    #[test]
    fn c_ident_vec_test() {
        use crate::Symbol;
        let def = "FZHL_DUSasb18 ";
        let val = vec![Symbol(span(def, Rule::ident).to_string())];
        assert_eq!(val, vec![Symbol("FZHL_DUSasb18".to_string())]);

        let def = "FZHL_DUSasb19,xkask_3298 ";
        let val = vec![Symbol(span(def, Rule::ident).to_string())];
        assert_eq!(val, vec![Symbol("FZHL_DUSasb19".to_string())],);
    }

    #[test]
    fn char_string_test() {
        let def = "\"ab\x00\x7f\"";
        let val = span(def, Rule::quoted_str);
        assert_eq!(val, "\"ab\x00\x7f\"");
    }

    #[test]
    fn attribute_value_f64_test() {
        let val = span("80.0", Rule::number).parse::<f64>().unwrap();
        assert!((val - 80.0).abs() < f64::EPSILON);
    }
}
