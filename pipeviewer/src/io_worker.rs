use std::fs::File;
use std::io::{self, BufReader, ErrorKind, Read, Result as IoResult, Write};

use crate::{Args, CHUNK_SIZE};

pub struct IoWorker<'m> {
    args: &'m Args,
}

impl<'m> IoWorker<'m> {
    pub fn new(args: &'m Args) -> Self {
        Self { args }
    }

    pub fn read(&self) -> IoResult<Vec<u8>> {
        let mut buffer = [0; CHUNK_SIZE];
        let num_read = self.reader()?.read(&mut buffer)?;
        Ok(Vec::from(&buffer[..num_read]))
    }

    pub fn write(&self, buffer: &[u8]) -> IoResult<bool> {
        if let Err(e) = self.writer()?.write_all(buffer) {
            if e.kind() == ErrorKind::BrokenPipe {
                // false means "stop the program cleanly"
                return Ok(false);
            }
            return Err(e);
        }
        // true means "keep going".
        Ok(true)
    }
}

impl<'m> IoWorker<'m> {
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
