// Copyright 2017-2018 by Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>
// The executable wrapper

extern crate changelog;
#[macro_use]
extern crate clap;
extern crate console;
extern crate env_logger;
extern crate failure;
#[macro_use]
extern crate log;

use changelog::{ChangeLog, Configuration, Result};
use clap::{App, AppSettings};
use console::style;
use env_logger::Builder;
use log::LevelFilter;
use std::env::args_os;
use std::ffi::OsString;
use std::io::Write;
use std::process::exit;

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

    // Pick overrides from the command line
    config.output.json = cli.is_present("json");
    let cmd = cli.value_of("remote").map(str::to_owned);
    config.output.remote = cmd.or(config.output.remote);
    let cmd = cli.value_of("template").map(str::to_owned);
    config.output.template = cmd.or(config.output.template);

    debug!("{:#?}", config);

    // Initialize the revision range
    let range = cli.values_of_lossy("range").unwrap_or_default();

    // Generate the change log for the range with the config
    let changelog = ChangeLog::from_log(range, &config);
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
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        n if n > 2 => LevelFilter::Trace,
        _ => LevelFilter::Warn,
    };

    // Build the logger and initialize
    Builder::from_default_env()
        .format(|buf, r| writeln!(buf, "{}: {}", r.level(), r.args()))
        .filter(Some("git_changelog"), level)
        .filter(Some("changelog"), level)
        .init();
}

#[cfg(test)]
mod tests {
    use failure::err_msg;
    use std::ffi::OsString;

    fn to_args(cmd: &str) -> Vec<OsString> {
        cmd.split_whitespace().map(OsString::from).collect()
    }

    #[test]
    fn run() {
        assert!(super::run(to_args("git-changelog v0.1.1..v0.2.1")).is_ok());
        assert!(super::run(to_args("git-changelog -d v0.1.1..v0.2.1")).is_ok());
        assert!(super::run(to_args("git-changelog -dd v0.1.1..v0.2.1")).is_ok());
        assert!(super::run(to_args("git-changelog -ddd -j")).is_ok());
    }

    #[test]
    fn show() {
        assert_eq!(
            super::show(super::run(to_args("git-changelog v0.1.1..v0.2.1"))),
            0
        );
        assert_eq!(super::show(Ok(String::from("foo"))), 0);
        assert_eq!(super::show(Err(err_msg("foo"))), 1);
    }
}
