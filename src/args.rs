use clap::{App, Arg};
use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_NAME: &str = env!("CARGO_PKG_NAME");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

pub struct Args {
    pub input_file: String,
    pub output_file: String,
    pub silent: bool,
}

impl Args {
    pub fn parse() -> Self {
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

        let input_file = matches
            .value_of("input-file")
            .unwrap_or_default()
            .to_string();
        let output_file = matches
            .value_of("output-file")
            .unwrap_or_default()
            .to_string();

        let silent = match matches.is_present("silent") {
            true => true,
            false => !env::var("PV_SILENT").unwrap_or_default().is_empty(),
        };

        return Self {
            input_file,
            output_file,
            silent,
        };
    }
}
