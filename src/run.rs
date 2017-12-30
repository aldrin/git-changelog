// Copyright 2017 Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

use git;
use report;
use output;
use commit;
use config;
use exitcode;

use serde_json;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use exitcode::ExitCode;

/// The tool entry-point
pub fn run(input: super::Input, cwd: &PathBuf) -> Result<String, ExitCode> {
    // First things first, are we even in a git repository?
    if git::in_git_repository().is_err() {
        return Err(exitcode::NOINPUT);
    }

    // Locate the configuration file and initialize configuration
    let lookup = || find_file(cwd.clone(), super::CONFIG_FILE_NAME);
    let config = load(input.config_file.or_else(lookup), config::from_yml)?;

    // Locate the template file and initialize Handlebars
    let lookup = || find_file(cwd.clone(), super::TEMPLATE_FILE_NAME);
    let hbs = load(input.output_template_file.or_else(lookup), output::from_hbs)?;

    // Decide the revision range we'll use for the report
    let range = match input.revision_range {
        // The user gave one, use it
        Some(vec) => vec,

        // None given
        None => {
            // Start from the last known tag
            let from = match git::last_tag() {
                Ok(Some(tag)) => tag,
                _ => {
                    // Go back one commit
                    warn!("No tags found, using HEAD^");
                    String::from("HEAD^")
                }
            };

            // Stop at HEAD
            let to = String::from("HEAD");

            // The default range
            vec![format!("^{}", from), to]
        }
    };

    // Show the revision
    info!("Using revision range {}", range.join(" "));

    // Get all commits in range
    let hashes = git::commits_in_range(&range);

    // We need a list of commits to make progress
    if hashes.is_err() {
        error!("No commits in range. {:?}", hashes);
        return Err(exitcode::NOINPUT);
    }

    // A list of all commits
    let mut commits = Vec::new();

    // Go through each sha in range
    for sha in hashes.unwrap() {
        // Get the commit message for the commit
        let message = git::get_commit_message(&sha);

        // If we cannot read it
        if message.is_err() {
            // Warn and ignore
            warn!("Commit {} could not be read. {:?}", sha, message);
            continue;
        }

        // Parse the commit
        let commit = commit::parse(&message.unwrap(), &config.date_format);

        // If it is an interesting commit
        if config::is_interesting(&config, &commit) {
            commits.push(commit);
        }
    }

    // Order the commits by time
    commits.sort_by(|a, b| a.time.cmp(&b.time));

    // Prepare the report
    let report = report::generate(&config, &commits);

    // Render the report
    let mut rendered = if input.output_json {
        serde_json::to_string_pretty(&report).unwrap()
    } else {
        output::render(&report, &hbs)
    };

    // If required, post-process
    if !config.post_processors.is_empty() {
        rendered = output::postprocess(&config.post_processors, &rendered);
    }

    // Done
    Ok(rendered)
}

type Loader<T> = fn(Option<String>) -> Result<T, Error>;

/// Load the given file using the loader
fn load<T>(given_file: Option<String>, loader: Loader<T>) -> Result<T, ExitCode> {
    // Get the name of the file for error reporting
    let file = given_file.clone().unwrap_or_else(|| "embedded".to_string());

    // Read the file
    match read_file(given_file) {
        // If the file cannot be opened
        Err(e) => {
            // Report the error
            error!("Failed to read from file '{:?}': {}", file, e);
            Err(exitcode::NOINPUT)
        }

        // If the read was OK, hand it to loader
        Ok(contents) => match loader(contents) {
            // If the loader fails
            Err(e) => {
                // Report the error
                error!("Invalid data in '{}': {}", file, e);
                Err(exitcode::DATAERR)
            }

            // If the loader succeeds
            Ok(item) => {
                info!("Read '{}'", file);

                // Return what we've got
                Ok(item)
            }
        },
    }
}

/// Identify the closest configuration file that should be used for this run
fn find_file(start_dir: PathBuf, file: &str) -> Option<String> {
    // Start at the current directory
    let mut cwd = start_dir;

    // While we have hope
    while cwd.exists() {
        // Set the filename we're looking for
        cwd.push(file);

        // If we find it
        if cwd.is_file() {
            // return it
            return Some(cwd.to_string_lossy().to_string());
        }

        // If not, remove the filename
        cwd.pop();

        // If we have room to go up
        if cwd.parent().is_some() {
            // Go up the path
            cwd.pop();
        } else {
            // Get out
            break;
        }
    }

    // No file found
    None
}

// Check if the chosen file is readable
fn read_file(chosen: Option<String>) -> Result<Option<String>, Error> {
    match chosen {
        Some(name) => {
            let mut s = String::new();
            File::open(name)?.read_to_string(&mut s)?;
            Ok(Some(s))
        }
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn find_file() {
        use super::find_file;
        use std::env::current_dir;
        assert!(find_file(current_dir().unwrap(), "Cargo.toml").is_some());
        assert!(find_file(current_dir().unwrap(), "unknown").is_none());
    }

    #[test]
    fn read_file() {
        use super::read_file;
        assert!(read_file(Some(String::from("Cargo.toml"))).is_ok());
        assert!(read_file(Some(String::from("unknown"))).is_err());
        assert!(read_file(Some(String::from("assets"))).is_err());
    }

    #[test]
    fn run() {
        use super::super::Input;
        use std::env::current_dir;
        assert!(super::run(Input::default(), &current_dir().unwrap()).is_ok());
    }
}
