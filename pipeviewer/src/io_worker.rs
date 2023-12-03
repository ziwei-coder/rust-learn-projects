use std::fs::File;
use std::io::{self, BufReader, ErrorKind, Read, Result as IoResult, Write};
use std::sync::{Arc, Mutex};

use crate::{Args, CHUNK_SIZE};

pub struct IoWorker {
    args: Args,
    read_quit: Arc<Mutex<bool>>,
    write_quit: Arc<Mutex<bool>>,
    stats_quit: Arc<Mutex<bool>>,
}

impl IoWorker {
    pub fn new(args: Args) -> Self {
        let quit = Arc::new(Mutex::new(false));
        let (read_quit, write_quit, stats_quit) = (quit.clone(), quit.clone(), quit.clone());

        Self {
            args,
            read_quit,
            write_quit,
            stats_quit,
        }
    }

    pub fn read_loop(&self) -> IoResult<()> {
        let mut buffer = [0; CHUNK_SIZE];

        loop {
            let num_read = match self.reader()?.read(&mut buffer) {
                Err(_) => break,
                Ok(0) => break,
                Ok(x) => x,
            };

            // todo: send this buffer to the stats thread
            let _data = Vec::from(&buffer[..num_read]);
        }

        // todo: send an empty buffer to the stats thread
        let mut quit = self.read_quit.lock().unwrap();
        *quit = true;

        Ok(())
    }

    pub fn write_loop(&self) -> IoResult<()> {
        loop {
            // todo: receive vector from stats thread
            let buffer: Vec<u8> = vec![];

            // If break drop the quit
            {
                let quit = self.write_quit.lock().unwrap();

                if *quit {
                    break;
                }
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

    pub fn stats_loop(&self) -> IoResult<()> {
        let mut total_bytes = 0;

        loop {
            // todd: receive the vector of bytes
            let buffer: Vec<u8> = vec![];
            total_bytes += buffer.len();

            if !self.args.silent {
                println!("\r{}", total_bytes);
            }

            // todo: send vector to write loop
            let quit = self.stats_quit.lock().unwrap();
            if *quit {
                break;
            }
        }

        println!();
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
}
