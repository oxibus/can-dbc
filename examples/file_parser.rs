use std::borrow::Cow;
use std::convert::TryFrom;
use std::fs;
use std::str::from_utf8;

use can_dbc::encodings::Encoding;
use can_dbc::{Dbc, Error};
use clap::Parser;

#[derive(Parser)]
#[command(about, version)]
struct Args {
    /// Input file encoding. If not specified, UTF-8 is assumed.
    #[arg(short, long, default_value = "utf-8")]
    encoding: Option<String>,

    /// DBC file path
    #[arg(default_value = "./examples/sample.dbc", value_name = "FILE")]
    input: String,
}

fn main() {
    let args = Args::parse();
    let path = &args.input;

    let data = fs::read(path).expect("Unable to read input file");

    let data = if let Some(enc) = &args.encoding {
        let enc = Encoding::for_label(enc.as_bytes()).expect("Unknown encoding");
        enc.decode_without_bom_handling_and_without_replacement(&data)
            .expect("Unable to decode using specified encoding")
    } else {
        let data = from_utf8(&data).expect("Input file is not valid UTF-8. Consider specifying the encoding with the --encoding option.");
        Cow::Borrowed(data)
    };

    match Dbc::try_from(data.as_ref()) {
        Ok(dbc_content) => println!("{dbc_content:#?}"),
        Err(e) => {
            match e {
                Error::Nom(nom::Err::Error(e) | nom::Err::Failure(e)) => eprintln!("{e:?}"),
                Error::Nom(nom::Err::Incomplete(needed)) => eprintln!("Nom incomplete needed: {needed:#?}"),
                Error::Incomplete(dbc, remaining) => eprintln!("Not all data in buffer was read {dbc:#?}, remaining unparsed (length: {}): {remaining}\n...(truncated)", remaining.len()),
                Error::MultipleMultiplexors => eprintln!("Multiple multiplexors defined"),
            }
        }
    }
}
