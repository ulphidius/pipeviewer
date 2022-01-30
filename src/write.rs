use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Result, Write};

pub fn write(output_file: &str, buffer: &[u8]) -> Result<bool> {
    let mut writer: Box<dyn Write> = match output_file.is_empty() {
        true => Box::new(BufWriter::new(io::stdout())),
        false => Box::new(BufWriter::new(File::create(output_file)?)),
    };
    if let Err(e) = writer.write_all(buffer) {
        if e.kind() == ErrorKind::BrokenPipe {
            // false means "stop program"
            return Ok(false);
        }
        return Err(e);
    };

    return Ok(true);
}
