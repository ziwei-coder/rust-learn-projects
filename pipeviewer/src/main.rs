use std::io::Result as IoResult;
use std::sync::Arc;
use std::thread;

use pipeviewer::{Args, IoWorker};

fn main() -> IoResult<()> {
    let args = Args::init();
    let worker = Arc::new(IoWorker::new(args));

    let (worker1, worker2) = (Arc::clone(&worker), Arc::clone(&worker));

    let read_handle = thread::spawn(move || worker1.read_loop());
    let stats_handle = thread::spawn(move || worker2.stats_loop());
    let write_handle = thread::spawn(move || worker.write_loop());

    // crash if any threads have crashed
    let read_result = read_handle.join().unwrap();
    let stats_result = stats_handle.join().unwrap();
    let write_result = write_handle.join().unwrap();

    // Return an error if any threads returned an error
    read_result?;
    stats_result?;
    write_result?;

    Ok(())
}
