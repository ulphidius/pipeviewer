use pipeviewer::{args::Args, read, stats, write};
use std::io::Result;

fn main() -> Result<()> {
    let args = Args::parse();
    let mut total_bytes = 0;

    loop {
        let buffer = match read::read(&args.input_file) {
            Ok(x) if x.is_empty() => break,
            Ok(x) => x,
            Err(_) => break,
        };
        stats::stats(args.silent, buffer.len(), &mut total_bytes, false);
        if !write::write(&args.output_file, &buffer)? {
            break;
        }
    }
    stats::stats(args.silent, 0, &mut total_bytes, true);
    Ok(())
}
