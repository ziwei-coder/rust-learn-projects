use std::io::{Read, Result as IoResult, Write};

use crate::io_worker::IoWorker;
use crate::matcher::Matcher;

mod io_worker;
mod matcher;

const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> IoResult<()> {
    let matcher = Matcher::init();
    let worker = IoWorker::new(&matcher);

    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];

    let num_read = match worker.reader()?.read(&mut buffer) {
        Err(_) | Ok(0) => return Ok(()),
        Ok(x) => x,
    };

    total_bytes += num_read;

    if !matcher.silent {
        print!("\r{}", total_bytes);
    }

    worker.writer()?.write_all(&buffer[..num_read])?;

    if !matcher.silent {
        println!("\rtotal_bytes: {}", total_bytes);
    }

    Ok(())
}
