//!
//! Module containing nom parser combinators
//!

use std::str;

use nom::{
    bytes::complete::{take_till, take_while, take_while1},
    character::complete::char,
    error::{ErrorKind, ParseError},
    multi::separated_list0,
    AsChar, IResult, InputTakeAtPosition,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_ident_test() {
        let def1 = "EALL_DUSasb18 ";
        let (_, cid1) = c_ident(def1).unwrap();
        assert_eq!("EALL_DUSasb18", cid1);

        let def2 = "_EALL_DUSasb18 ";
        let (_, cid2) = c_ident(def2).unwrap();
        assert_eq!("_EALL_DUSasb18", cid2);

        // identifiers must not start with digit1s
        let def3 = "3EALL_DUSasb18 ";
        let cid3_result = c_ident(def3);
        assert!(cid3_result.is_err());
    }

    #[test]
    fn c_ident_vec_test() {
        let cid = "FZHL_DUSasb18 ";
        let (_, cid1) = c_ident_vec(cid).unwrap();

        assert_eq!(vec!("FZHL_DUSasb18".to_string()), cid1);

        let cid_vec = "FZHL_DUSasb19,xkask_3298 ";
        let (_, cid2) = c_ident_vec(cid_vec).unwrap();

        assert_eq!(
            vec!("FZHL_DUSasb19".to_string(), "xkask_3298".to_string()),
            cid2
        );
    }

    #[test]
    fn char_string_test() {
        let def = "\"ab\x00\x7f\"";
        let (_, char_string) = char_string(def).unwrap();
        let exp = "ab\x00\x7f";
        assert_eq!(exp, char_string);
    }
}

pub(crate) fn is_semi_colon(chr: char) -> bool {
    chr == ';'
}

pub(crate) fn is_c_string_char(chr: char) -> bool {
    chr.is_ascii_digit() || chr.is_alphabetic() || chr == '_'
}

pub(crate) fn is_c_ident_head(chr: char) -> bool {
    chr.is_alphabetic() || chr == '_'
}

pub(crate) fn is_quote(chr: char) -> bool {
    chr == '"'
}

/// Multispace zero or more
pub(crate) fn ms0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
    input.split_at_position_complete(|item| {
        let c = item.as_char();
        c != ' '
    })
}

/// Multi space one or more
pub(crate) fn ms1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar + Clone,
{
    input.split_at_position1_complete(
        |item| {
            let c = item.as_char();
            c != ' '
        },
        ErrorKind::MultiSpace,
    )
}

/// Colon aka `:`
pub(crate) fn colon(s: &str) -> IResult<&str, char> {
    char(':')(s)
}

/// Comma aka ','
pub(crate) fn comma(s: &str) -> IResult<&str, char> {
    char(',')(s)
}

/// Comma aka ';'
pub(crate) fn semi_colon(s: &str) -> IResult<&str, char> {
    char(';')(s)
}

/// Quote aka '"'
pub(crate) fn quote(s: &str) -> IResult<&str, char> {
    char('"')(s)
}

/// Pipe character
pub(crate) fn pipe(s: &str) -> IResult<&str, char> {
    char('|')(s)
}

/// at character
pub(crate) fn at(s: &str) -> IResult<&str, char> {
    char('@')(s)
}

/// brace open aka '('
pub(crate) fn brc_open(s: &str) -> IResult<&str, char> {
    char('(')(s)
}

/// brace close aka ')'
pub(crate) fn brc_close(s: &str) -> IResult<&str, char> {
    char(')')(s)
}

/// bracket open aka '['
pub(crate) fn brk_open(s: &str) -> IResult<&str, char> {
    char('[')(s)
}

/// bracket close aka ']'
pub(crate) fn brk_close(s: &str) -> IResult<&str, char> {
    char(']')(s)
}

/// A valid C_identifier. C_identifiers start with a  alphacharacter or an underscore
/// and may further consist of alpha­numeric, characters and underscore
pub(crate) fn c_ident(s: &str) -> IResult<&str, String> {
    let (s, head) = take_while1(is_c_ident_head)(s)?;
    let (s, remaining) = take_while(is_c_string_char)(s)?;
    Ok((s, [head, remaining].concat()))
}

pub(crate) fn c_ident_vec(s: &str) -> IResult<&str, Vec<String>> {
    separated_list0(comma, c_ident)(s)
}

pub(crate) fn char_string(s: &str) -> IResult<&str, &str> {
    let (s, _) = quote(s)?;
    let (s, char_string_value) = take_till(is_quote)(s)?;
    let (s, _) = quote(s)?;
    Ok((s, char_string_value))
}
