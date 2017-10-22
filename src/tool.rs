// Copyright 2017 Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

use git;
use report;
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

    // Inform the curious
    config.categories.show("category", true);
    config.scopes.show("scope", false);

    // Decide the revision range we'll use for the report
    let range = match given_range {
        Some(vec) => vec,
        None => {
            let from = match git::last_tag() {
                Ok(Some(tag)) => tag,
                _ => {
                    warn!("No tags found, using HEAD^");
                    String::from("HEAD^")
                }
            };
            let to = String::from("HEAD");
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
        let commit = commit::parse(&message.unwrap(), &config.report.date_format);

        // If it is an interesting commit
        if config.is_interesting(&commit) {
            commits.push(commit);
        }
    }

    // Prepare the report
    let report = report::generate(&commits, config);

    // Show the report
    println!("{}", report::render(&report, config));

    // Done
    exit::OK
}
