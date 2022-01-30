use crossbeam::channel::{bounded, unbounded};
use pipeviewer::{args::Args, read, stats, write};
use std::io::Result;
use std::thread;

fn main() -> Result<()> {
    let args = Args::parse();

    let (stats_tx, stats_rx) = unbounded();
    let (write_tx, write_rx) = bounded(1024);

    let read_handle = thread::spawn(move || read::read_loop(&args.input_file, stats_tx, write_tx));
    let stats_handle = thread::spawn(move || stats::stats_loop(args.silent, stats_rx));
    let write_handle = thread::spawn(move || write::write_loop(&args.output_file, write_rx));

    // crash if threads have crashed
    // Join all finished threads
    let read_io_result = read_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();

    read_io_result?;
    stats_io_result?;
    write_io_result?;

    return Ok(());
}
