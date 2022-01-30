use crate::CHUNK_SIZE;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};

pub fn read(input_file: &str) -> Result<Vec<u8>> {
    let mut reader: Box<dyn Read> = match input_file.is_empty() {
        true => Box::new(BufReader::new(io::stdin())),
        false => Box::new(BufReader::new(File::open(input_file)?)),
    };

    let mut buffer = [0; CHUNK_SIZE];
    let num_read = reader.read(&mut buffer)?;

    return Ok(buffer[..num_read].to_vec());
}
