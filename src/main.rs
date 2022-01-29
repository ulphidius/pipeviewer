use clap::{App, Arg};
use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, ErrorKind, Read, Result, Write};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_NAME: &str = env!("CARGO_PKG_NAME");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
    let matches = App::new(APP_NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about("application which count number of bytes from stdin or from a file")
        .long_about(DESCRIPTION)
        .arg(Arg::new("input-file").help("Read from a file instead of stdin"))
        .arg(
            Arg::new("output-file")
                .short('o')
                .long("output-file")
                .takes_value(true)
                .help("Write output to a file instead of stdout"),
        )
        .arg(Arg::new("silent").short('s').long("silent"))
        .get_matches();

    let input_file = matches.value_of("input-file").unwrap_or_default();
    let output_file = matches.value_of("output-file").unwrap_or_default();

    let silent = match matches.is_present("silent") {
        true => true,
        false => !env::var("PV_SILENT").unwrap_or_default().is_empty(),
    };
    let mut reader: Box<dyn Read> = match input_file.is_empty() {
        true => Box::new(BufReader::new(io::stdin())),
        false => Box::new(BufReader::new(File::open(input_file)?)),
    };
    let mut writer: Box<dyn Write> = match output_file.is_empty() {
        true => Box::new(BufWriter::new(io::stdout())),
        false => Box::new(BufWriter::new(File::create(output_file)?)),
    };

    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];

    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;
        if !silent {
            eprint!("\r{}", total_bytes);
        }
        if let Err(e) = writer.write_all(&buffer[..num_read]) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);
        };
    }

    Ok(())
}
