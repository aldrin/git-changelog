// Copyright 2017 Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

extern crate log;
extern crate clap;
extern crate changelog;
extern crate env_logger;

use std::fs::File;
use clap::{Arg, App};
use std::error::Error;
use std::process::exit;
use env_logger::LogBuilder;
use log::{LogRecord, LogLevelFilter};

fn main() {

    // Initialize the CLI
    let cli = App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(&format!("(v{})", env!("CARGO_PKG_VERSION"))[..])
        .arg(Arg::with_name("revision-range").multiple(true).help(
            "The revision range, defaults to HEAD...<last-tag>",
        ))
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .value_name("FILE")
                .validator(|s| valid_path(&s))
                .help("Configuration file"),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .multiple(true)
                .help("Prints debug logs"),
        )
        .get_matches();

    // Initialize log verbosity
    init_logging(cli.occurrences_of("debug"));

    // Identify the configuration file we're going to use
    let filename = changelog::config::find_file(cli.value_of("config"));

    // Initialize the configuration and run the tool
    let result = match changelog::config::from(&filename) {
        Ok(c) => changelog::tool::run(&c, cli.values_of_lossy("revision-range")),
        _ => -1,
    };

    // Done
    exit(result);
}

/// Initialize error reporting.
fn init_logging(verbosity: u64) {
    // Pick a log level
    let level = match verbosity {
        1 => LogLevelFilter::Info,
        2 => LogLevelFilter::Debug,
        n if n > 2 => LogLevelFilter::Trace,
        _ => LogLevelFilter::Warn,
    };

    // Pick the log format
    let format = |r: &LogRecord| format!("{}: {}", r.level(), r.args());
    // Build the logger
    let mut builder = LogBuilder::new();
    builder.format(format).filter(None, level);
    builder.init().unwrap();
}

/// Check if the given path is valid
fn valid_path(path: &str) -> Result<(), String> {
    File::open(path).map(|_| ()).map_err(|e| {
        let mut reason = String::from("Invalid file: ");
        reason.push_str(path);
        reason.push_str(" Reason: ");
        reason.push_str(e.description());
        reason
    })
}
