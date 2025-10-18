use std::convert::TryFrom;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use clap::Parser;

#[derive(Parser)]
#[command(about, version)]
struct Args {
    /// DBC file path
    #[arg(
        short,
        long,
        default_value = "./examples/sample.dbc",
        value_name = "FILE"
    )]
    input: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let path = &args.input;

    let mut f = File::open(path)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    let dbc_in = std::str::from_utf8(&buffer).unwrap();

    match can_dbc::Dbc::try_from(dbc_in) {
        Ok(dbc_content) => println!("DBC Content{dbc_content:#?}"),
        Err(e) => {
            match e {
                can_dbc::Error::Nom(nom::Err::Error(e) | nom::Err::Failure(e)) => eprintln!("{e:?}"),
                can_dbc::Error::Nom(nom::Err::Incomplete(needed)) => eprintln!("Nom incomplete needed: {needed:#?}"),
                can_dbc::Error::Incomplete(dbc, remaining) => eprintln!("Not all data in buffer was read {dbc:#?}, remaining unparsed (length: {}): {remaining}\n...(truncated)", remaining.len()),
                can_dbc::Error::MultipleMultiplexors => eprintln!("Multiple multiplexors defined"),
                can_dbc::Error::InvalidContent(e) => eprintln!("Invalid content: {e:?}"),
            }
        }
    }

    Ok(())
}
