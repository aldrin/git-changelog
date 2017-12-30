// Copyright 2017 Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

extern crate changelog;
#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate log;

use clap::App;
use std::env::args_os;
use std::env::current_dir;
use std::process::exit;
use std::ffi::OsString;
use env_logger::LogBuilder;
use log::{LogLevelFilter, LogRecord};

// The entry-point
fn main() {
    run(args_os().collect());
}

// The real entry-point
fn run(args: Vec<OsString>) {
    // Initialize the CLI
    let yml = load_yaml!("assets/cli.yml");
    let cli = App::from_yaml(yml)
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches_from(args);

    // Initialize log verbosity
    let level = match cli.occurrences_of("debug") {
        1 => LogLevelFilter::Info,
        2 => LogLevelFilter::Debug,
        n if n > 2 => LogLevelFilter::Trace,
        _ => LogLevelFilter::Warn,
    };

    // Pick the log format
    let format = |r: &LogRecord| format!("{}: {}", r.level(), r.args());

    // Build the logger
    let mut builder = LogBuilder::new();
    builder.format(format).filter(Some("changelog"), level);
    builder.init().unwrap();

    // Initialize the tool from the cli arguments
    let input = changelog::Input {
        output_json: cli.is_present("json"),
        revision_range: cli.values_of_lossy("RANGE"),
        config_file: cli.value_of("config").map(str::to_string),
        output_template_file: cli.value_of("template").map(str::to_string),
    };

    // Done
    exit(match changelog::run(input, &current_dir().unwrap()) {
        Err(exit) => exit,
        Ok(output) => {
            println!("{}", output);
            0
        }
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn just_run() {
        super::run(vec![]);
    }
}
