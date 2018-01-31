// Copyright 2017-2018 by Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

/// This crate implements an executable wrapper around the `changelog` crate.
extern crate changelog;
#[macro_use]
extern crate clap;
extern crate console;
extern crate env_logger;
extern crate failure;
#[macro_use]
extern crate log;

use clap::{App, AppSettings};
use std::env::args_os;
use std::ffi::OsString;
use std::process::exit;
use console::style;
use env_logger::LogBuilder;
use log::{LogLevelFilter, LogRecord};
use changelog::{ChangeLog, Configuration, Result};

/// The entry-point.
fn main() {
    exit(show(run(args_os().collect())));
}

/// The real entry-point.
fn run(args: Vec<OsString>) -> Result<String> {
    // Load the CLI definition from the YAML asset
    let yml = load_yaml!("assets/git-changelog.yml");
    let app = App::from_yaml(yml)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DisableVersion)
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!());

    // Process the arguments CLI application
    let cli = app.get_matches_from(args);

    // Initialize log verbosity
    initialize_logging(cli.occurrences_of("debug"));

    // Ensure we're in a git directory
    changelog::in_git_repository()?;

    // Initialize the tool configuration
    let mut config = Configuration::from_file(cli.value_of("config"))?;
    trace!("{:#?}", config);

    // Pick overrides from the command line
    config.output.json = cli.is_present("json");
    config.output.remote = cli.value_of("remote").map(str::to_owned);
    config.output.template = cli.value_of("template").map(str::to_owned);

    // Initialize the revision range
    let range = cli.values_of_lossy("range").unwrap_or_default().join(" ");

    // Generate the change log for the range with the config
    let changelog = ChangeLog::from_range(range, &config);
    trace!("{:#?}", changelog);

    // Render the change log with the given output choices
    changelog::render(&changelog, &config.output)
}

/// The output routine. Just print for now.
fn show(result: Result<String>) -> i32 {
    match result {
        Ok(out) => {
            for line in out.lines() {
                if line.starts_with('#') {
                    let (header, heading) = line.split_at(line.find(' ').unwrap_or(0));
                    println!("{}{}", style(header).bold().cyan(), style(heading).cyan())
                } else {
                    println!("{}", line)
                }
            }
            0
        }
        Err(e) => {
            error!("{}", e);
            1
        }
    }
}

/// Initialize tool logging with the given verbosity.
fn initialize_logging(verbosity: u64) {
    // Pick the log level
    let level = match verbosity {
        1 => LogLevelFilter::Info,
        2 => LogLevelFilter::Debug,
        n if n > 2 => LogLevelFilter::Trace,
        _ => LogLevelFilter::Warn,
    };

    // Pick the log format
    let format = |r: &LogRecord| format!("{}: {}", r.level(), r.args());

    // Build the logger and initialize
    LogBuilder::new()
        .filter(Some("git_changelog"), level)
        .filter(Some("changelog"), level)
        .format(format)
        .init()
        .ok();
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;
    use failure::err_msg;

    fn to_args(cmd: &str) -> Vec<OsString> {
        cmd.split_whitespace().map(OsString::from).collect()
    }

    #[test]
    fn run() {
        assert!(super::run(to_args("git-changelog -d -j")).is_ok());
        assert!(super::run(to_args("git-changelog -dd v0.1.1..v0.2.1")).is_ok());
    }

    #[test]
    fn show() {
        assert_eq!(super::show(Ok(String::from("foo"))), 0);
        assert_eq!(super::show(Err(err_msg("foo"))), 1);
    }
}
