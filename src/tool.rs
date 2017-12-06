// Copyright 2017 Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

use git;
use report;
use output;
use commit;
use config;

/// The exit code catalog
mod exit {
    /// All well
    pub const OK: i32 = 0;

    /// Not in a git repository
    pub const NOT_GIT: i32 = 1;

    /// No commits were found in range
    pub const NO_COMMITS: i32 = 2;
}

/// The main tool entry-point
pub fn run(config: &config::Configuration, given_range: Option<Vec<String>>) -> i32 {

    // First things first, are we even in a git repository?
    if git::in_git_repository().is_err() {
        return exit::NOT_GIT;
    }

    // Decide the revision range we'll use for the report
    let range = match given_range {

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
        return exit::NO_COMMITS;
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
        if config::is_interesting(config, &commit) {
            commits.push(commit);
        }
    }

    // Prepare the report
    let report = report::generate(config, &commits);

    // Show the report
    output::render(config, &report);

    // Done
    exit::OK
}
