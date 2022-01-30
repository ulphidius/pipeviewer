use crate::CHUNK_SIZE;
use crossbeam::channel::Sender;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};

pub fn read_loop(
    input_file: &str,
    stats_tx: Sender<usize>,
    write_tx: Sender<Vec<u8>>,
) -> Result<()> {
    let mut reader: Box<dyn Read> = match input_file.is_empty() {
        true => Box::new(BufReader::new(io::stdin())),
        false => Box::new(BufReader::new(File::open(input_file)?)),
    };

    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        let _ = stats_tx.send(num_read);
        if write_tx.send(buffer[..num_read].to_vec()).is_err() {
            break;
        }
    }

    let _ = stats_tx.send(0);
    let _ = write_tx.send(Vec::new());

    return Ok(());
}
