use std::fs::File;
use std::io::{self, Read, Result as IoResult, Write};

use crate::matcher::Matcher;

pub struct IoWorker<'m> {
    matcher: &'m Matcher,
}

impl<'m> IoWorker<'m> {
    pub fn new(matcher: &'m Matcher) -> Self {
        Self { matcher }
    }

    pub fn reader(&self) -> IoResult<Box<dyn Read>> {
        match self.matcher.infile.as_ref() {
            Some(path) => {
                let file = File::open(path)?;
                Ok(Box::new(file))
            }
            None => Ok(Box::new(io::stdin())),
        }
    }

    pub fn writer(&self) -> IoResult<Box<dyn Write>> {
        match self.matcher.outfile.as_ref() {
            Some(path) => {
                let file = File::create(path)?;
                Ok(Box::new(file))
            }
            None => Ok(Box::new(io::stdout())),
        }
    }
}
