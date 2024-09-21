use can_dbc::DBC;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let read_dbc = DBC::read_from_file("./examples/sample.dbc")?;
    read_dbc.write_to_file("./examples/sample_output.dbc")?;
    Ok(())
}