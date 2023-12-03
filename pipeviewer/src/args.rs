use std::env;
use std::ops::Deref;

use clap::{App, Arg, ArgMatches};

#[derive(Clone)]
pub struct Args {
    pub infile: Option<String>,
    pub outfile: Option<String>,
    pub silent: bool,
}

impl Args {
    pub fn init() -> Self {
        let matches = Matches::new();

        Self {
            infile: matches.get("infile"),
            outfile: matches.get("outfile"),
            silent: matches.is_silent(),
        }
    }
}

struct Matches<'a>(ArgMatches<'a>);

impl<'a> Matches<'a> {
    fn new() -> Self {
        let matches = App::new("pipeviewer")
            .arg(Arg::with_name("infile").help("Read from a file instead of stdin"))
            .arg(
                Arg::with_name("outfile")
                    .short("o")
                    .long("outfile")
                    .takes_value(true)
                    .help("Write to a file instead of stdout"),
            )
            .arg(Arg::with_name("silent").short("s").long("silent"))
            .get_matches();

        Self(matches)
    }
}

impl<'a> Matches<'a> {
    fn get(&self, key: &str) -> Option<String> {
        self.value_of(key).map(|s| s.to_owned())
    }

    fn is_silent(&self) -> bool {
        if self.is_present("silent") {
            true
        } else {
            !env::var("PV_SILENT").unwrap_or_default().is_empty()
        }
    }
}

impl<'a> Deref for Matches<'a> {
    type Target = ArgMatches<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
