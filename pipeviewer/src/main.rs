use std::io::Result as IoResult;

use pipeviewer::{stats, Args, IoWorker};

fn main() -> IoResult<()> {
    let args = Args::init();
    let worker = IoWorker::new(&args);

    let mut total_bytes = 0;

    loop {
        let buffer = match worker.read() {
            Ok(x) if x.is_empty() => break,
            Ok(x) => x,
            Err(_) => break,
        };

        stats(args.silent, buffer.len(), &mut total_bytes);

        if !worker.write(&buffer)? {
            break;
        };
    }

    Ok(())
}
