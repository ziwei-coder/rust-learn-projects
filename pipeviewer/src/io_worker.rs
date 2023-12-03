use std::fs::File;
use std::io::{self, BufReader, ErrorKind, Read, Result as IoResult, Write};

use crossbeam::channel::{Receiver, Sender};

use crate::{Args, CHUNK_SIZE};

pub struct IoWorker {
    args: Args,
}

impl IoWorker {
    pub fn new(args: Args) -> Self {
        Self { args }
    }

    pub fn read_loop(&self, stats_tx: Sender<usize>, write_tx: Sender<Vec<u8>>) -> IoResult<()> {
        let mut buffer = [0; CHUNK_SIZE];

        loop {
            let num_read = match self.reader()?.read(&mut buffer) {
                Err(_) => break,
                Ok(0) => break,
                Ok(x) => x,
            };

            // Send the stats to stats loop, and send the data to write loop
            let _ = stats_tx.send(num_read);
            if write_tx.send(Vec::from(&buffer[..num_read])).is_err() {
                break;
            }
        }

        // Send the stats to stats loop, and send the data to write loop
        let _ = stats_tx.send(0);
        let _ = write_tx.send(Vec::new());

        Ok(())
    }

    pub fn write_loop(&self, write_rx: Receiver<Vec<u8>>) -> IoResult<()> {
        loop {
            // Receive data from the read loop
            let buffer = write_rx.recv().unwrap();

            if buffer.is_empty() {
                break;
            }

            if let Err(e) = self.writer()?.write_all(&buffer) {
                if e.kind() == ErrorKind::BrokenPipe {
                    // false means "stop the program cleanly"
                    return Ok(());
                }
                return Err(e);
            }
        }

        Ok(())
    }

    pub fn stats_loop(&self, stats_rx: Receiver<usize>) -> IoResult<()> {
        let mut total_bytes = 0;

        loop {
            // Receive the data bytes from the read loop
            let num_bytes = stats_rx.recv().unwrap();
            total_bytes += num_bytes;

            if !self.is_silent() {
                print!("\r{}", total_bytes);
            }

            if num_bytes == 0 {
                break;
            }
        }

        if !self.is_silent() {
            println!();
        }

        Ok(())
    }
}

impl IoWorker {
    fn reader(&self) -> IoResult<Box<dyn Read>> {
        match self.args.infile.as_ref() {
            Some(path) => {
                let file = BufReader::new(File::open(path)?);
                Ok(Box::new(file))
            }
            None => Ok(Box::new(BufReader::new(io::stdin()))),
        }
    }

    fn writer(&self) -> IoResult<Box<dyn Write>> {
        match self.args.outfile.as_ref() {
            Some(path) => {
                let file = File::create(path)?;
                Ok(Box::new(file))
            }
            None => Ok(Box::new(io::stdout())),
        }
    }

    fn is_silent(&self) -> bool {
        self.args.silent
    }
}
