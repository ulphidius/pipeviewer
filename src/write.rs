use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Result, Write};
use std::sync::mpsc::Receiver;

pub fn write_loop(output_file: &str, write_rx: Receiver<Vec<u8>>) -> Result<()> {
    let mut writer: Box<dyn Write> = match output_file.is_empty() {
        true => Box::new(BufWriter::new(io::stdout())),
        false => Box::new(BufWriter::new(File::create(output_file)?)),
    };
    
    loop {
        let buffer: Vec<u8> = write_rx.recv().unwrap();
        if buffer.is_empty() {
            break;
        }

        if let Err(e) = writer.write_all(&buffer) {
            if e.kind() == ErrorKind::BrokenPipe {
                return Ok(());
            }
            return Err(e);
        };
    }

    return Ok(());
}
