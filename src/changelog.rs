// Copyright 2017-2018 by Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

use chrono::prelude::*;
use chrono::MIN_DATE;
use commit::{Commit, CommitList, Line};
use git;
use input::{Configuration, Conventions};
use std::collections::HashMap;
use std::str;

/// A categorized changelog
#[derive(Debug, Default, Serialize, Eq, PartialEq)]
pub struct ChangeLog {
    /// A list of scoped changes in the commit range.
    pub scopes: Vec<Scope>,

    /// A list of "interesting" commits in the range.
    pub commits: Vec<Commit>,

    /// The fetch url of the remote (useful for change number links)
    pub remote_url: Option<String>,

    /// The revision range for commits in this changelog
    pub range: String,

    /// The time range for the commits in this changelog
    pub date: String,
}

/// Changes grouped by scope (e.g. "API", "Documentation", etc.).
#[derive(Debug, Default, Serialize, Eq, PartialEq)]
pub struct Scope {
    /// The title of the scope, as defined in [`Conventions`](struct.Conventions.html).
    pub title: String,

    /// A list of categorized changes in this scope
    pub categories: Vec<Category>,
}

/// Changes grouped by categories (e.g. "Fixes", "Breaking Changes", etc.).
#[derive(Debug, Default, Serialize, Eq, PartialEq)]
pub struct Category {
    /// The title of the category, as defined in [`Conventions`](struct.Conventions.html).
    pub title: String,

    /// A list of changes in this category groups across all commits in range.
    pub changes: Vec<String>,
}

impl ChangeLog {
    /// Generate a new changelog for the default input range
    pub fn new() -> Self {
        Self::from_log(Vec::new(), &Configuration::new())
    }

    /// Generate a changelog for the given range
    pub fn from_range(range: &str, config: &Configuration) -> Self {
        Self::from_log(vec![range.to_string()], config)
    }

    /// Create a changelog from the given `git log` arguments
    pub fn from_log(mut args: Vec<String>, config: &Configuration) -> Self {
        // The default `git log` behavior is to list _all_ commits
        if args.is_empty() {
            // The default `git changelog` behavior is to list _all_ commits since last tag.
            if let Ok(Some(tag)) = git::last_tag() {
                args.push(format!("{}..HEAD", tag))
            } else {
                // If there are no tags, default to the last commit
                args.push(String::from("HEAD^..HEAD"))
            }
        }

        let header = args.join(" ");
        let range = CommitList::from(args);
        info!("Using revision range '{}'", range);

        // Compute the change log
        let mut log = Self::from(range, config);

        // Record the range we used (it is used by the template)
        log.range = header;

        // Done.
        log
    }

    /// Create a changelog from the given commits using the given conventions
    pub fn from<T: Iterator<Item = Commit>>(commits: T, config: &Configuration) -> Self {
        // Initialize a intermediate raw report
        let mut raw = RawReport::new();

        // Initialize the final change log
        let mut changelog = ChangeLog::default();

        // Walk through each commit in the range
        for commit in commits {
            // Offer it to the raw report
            if raw.add(&commit, &config.conventions) {
                // Inform the user we're picking this one
                trace!("Interesting commit {}", commit);

                // Add it to the final list
                changelog.commits.push(commit);
            } else {
                // Inform the user we're ignoring this one
                debug!("No interesting changes in commit {}", &commit);
            }
        }

        // Prepare the final report
        for scope in config.conventions.scope_titles() {
            if let Some(mut categorized) = raw.slots.remove(&scope) {
                let title = scope.to_owned();
                let mut categories = Vec::new();

                for category in config.conventions.category_titles() {
                    let title = category.to_owned();
                    if let Some(changes) = categorized.remove(&category) {
                        categories.push(Category { title, changes });
                    }
                }
                changelog.scopes.push(Scope { title, categories })
            }
        }

        // Add the remote url, if we have one (it's used by links to commits and PRs)
        let remote = match config.output.remote {
            Some(ref r) => r,
            None => "origin",
        };
        changelog.remote_url = git::get_remote_url(remote).unwrap_or(None);

        // Add the last change date
        changelog.date = raw.date.format("%Y-%m-%d").to_string();

        changelog
    }
}

/// Raw report
struct RawReport<'a> {
    /// The date of the last change in the range
    date: Date<Utc>,
    /// Placeholder slots for aggregation
    slots: HashMap<&'a str, HashMap<&'a str, Vec<String>>>,
}

impl<'a> RawReport<'a> {
    /// Initialize a new report
    fn new() -> Self {
        Self {
            date: MIN_DATE,
            slots: HashMap::default(),
        }
    }

    /// A the given commit to the report with the given conventions
    fn add<'c>(&mut self, commit: &'c Commit, conventions: &'a Conventions) -> bool {
        // Track if this commit brought anything interesting
        let mut interesting = false;

        // The running current line
        let mut current = Line::default();

        // Take each line
        for line in commit {
            // If the line is categorized
            if line.category.is_some() {
                // close the current active line
                interesting |= self.record(current, conventions);

                // and reset it to a clean slate
                current = Line::default();
                current.scope = line.scope;
                current.category = line.category;
            }

            // If we don't have any text yet
            if current.text.is_none() {
                // Initialize it with this line's text
                current.text = line.text
            } else if let Some(text) = current.text.as_mut() {
                // Append this line text to the current text
                text.push('\n');
                text.push_str(&line.text.unwrap_or_default());
            }
        }

        // We've read all the lines, close the running current
        interesting |= self.record(current, conventions);

        // Update the report time
        if let Ok(time) = DateTime::parse_from_rfc2822(&commit.time) {
            // Normalize commit timezones
            let date = time.with_timezone(&Utc).date();

            // If the commit date is after the current last date
            if date > self.date {
                // Update the report date
                self.date = date;
            }
        }

        // Done
        interesting
    }

    /// Record the current line into the report
    fn record(&mut self, current: Line, conventions: &'a Conventions) -> bool {
        // Get the titles and for the current scope and category
        let scope = conventions.scope_title(current.scope);
        let category = conventions.category_title(current.category);

        // If the titles are missing, the user is not interested in these changes
        let interesting = category.is_some() && scope.is_some() && current.text.is_some();

        // If the line is interesting
        if interesting {
            // Put it in its place
            self.slots
                .entry(scope.unwrap())
                .or_insert_with(HashMap::new)
                .entry(category.unwrap())
                .or_insert_with(Vec::new)
                .push(current.text.unwrap());
        }

        // Done
        interesting
    }
}
