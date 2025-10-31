use crate::ast::Dbc;

/// Possible error cases for `can-dbc`
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Error<'a> {
    /// Remaining String, the DBC was only read partially.
    /// Occurs when e.g. an unexpected symbol occurs.
    Incomplete(Dbc, &'a str),
    /// Parser failed
    Nom(nom::Err<nom::error::Error<&'a str>>),
    /// Can't Lookup multiplexors because the message uses extended multiplexing.
    MultipleMultiplexors,
}
