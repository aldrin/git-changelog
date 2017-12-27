// Copyright 2017 Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

//! `changelog` is the underlying library behind the CLI tool `git-changelog`.
//!
//! The tool automates repository changelog generation without enforcing any git workflow
//! conventions. When developers wish to record a "user visible" change to the repository (e.g. new
//! feature, bug fix, breaking change, etc.) they can tag lines in their commit message with a few
//! keywords. These tags are then used to organize changes made to the repository into *scopes* and
//! *categories* and these organized changes can then be presented as pretty change-logs or release
//! notes. Commits messages without tags are quietly ignored and developers are free to tag as
//! little or as much as they want.
extern crate chrono;
extern crate exitcode;
extern crate handlebars;
#[macro_use]
extern crate log;
#[macro_use]
extern crate nom;
extern crate regex;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_yaml;

mod git;
mod run;
mod commit;
mod config;
mod report;
mod output;

/// The default configuration file name
pub const CONFIG_FILE_NAME: &str = ".changelog.yml";

/// The default template file name
pub const TEMPLATE_FILE_NAME: &str = ".changelog.hbs";

/// Tool input
#[derive(Default)]
pub struct Input {
    /// Configuration file to use
    pub config_file: Option<String>,

    /// Git revision range
    pub revision_range: Option<Vec<String>>,

    /// Render report as JSON
    pub output_json: bool,

    /// The handlebar template file
    pub output_template_file: Option<String>,
}

/// A tag definition
#[serde(default)]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Tag {
    /// The identifying keyword
    pub keyword: String,

    /// The report heading
    pub title: String,
}

/// A post-processor definition
#[serde(default)]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PostProcessor {
    /// The lookup pattern
    pub lookup: String,

    /// The replace pattern
    pub replace: String,
}

/// The tool configuration structure (can be specified in a file)
#[serde(default)]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Configuration {
    /// The change category configuration
    pub categories: Vec<Tag>,

    /// The change scope configuration
    pub scopes: Vec<Tag>,

    /// The date format
    pub date_format: String,

    /// The line post-processors
    pub post_processors: Vec<PostProcessor>,
}

/// The complete report
#[derive(Serialize, Debug)]
pub struct ChangeLog<'a> {
    /// Scoped changes in the report
    pub scopes: Vec<Scope>,

    /// All interesting commits
    pub commits: &'a [Commit],
}

/// A group of changes in the same scope
#[derive(Serialize, Debug)]
pub struct Scope {
    /// The title of the scope
    pub title: String,

    /// A list of categorized changes
    pub categories: Vec<Category>,
}

/// A group of changes with the same category
#[derive(Serialize, Debug)]
pub struct Category {
    /// The title of the category
    pub title: String,

    /// A list of change descriptions
    pub changes: Vec<Text>,
}

/// Change description
#[derive(Serialize, Clone, Debug)]
pub struct Text {
    /// A sequence number to inform ordering
    pub sequence: u32,

    /// An opening headline
    pub opening: String,

    /// The remaining lines in the description
    pub rest: Vec<String>,
}

/// A single commit
#[derive(Serialize, Debug, Default)]
pub struct Commit {
    /// The change SHA
    pub sha: String,

    /// The change author
    pub author: String,

    /// The change timestamp
    pub time: String,

    /// The change summary
    pub summary: String,

    /// The change number
    pub number: Option<u32>,

    /// The message lines
    pub lines: Vec<Line>,
}

/// A single line in the change message
#[derive(Default, Serialize, Debug)]
pub struct Line {
    /// The scope
    pub scope: Option<String>,

    /// The category
    pub category: Option<String>,

    /// The text
    pub text: Option<String>,
}

// The entrypoint
pub use run::run;

#[cfg(test)]
mod tests {

    fn fake_commit() -> super::Commit {
        let header = vec![
            "2e51cdb3ef163acd31ad0ae9d1b861d544f8162b",
            "aaaaaa a a'aaaaa",
            "Sun, 22 Oct 2017 17:26:56 -0400",
        ];
        let message = include_str!("assets/sample-commit.message").lines();

        let mut lines: Vec<String> = Vec::new();
        for l in header {
            lines.push(l.to_string());
        }
        for l in message {
            lines.push(l.to_string());
        }

        super::commit::parse(&lines, "%Y-%m-%d %H:%M")
    }

    #[test]
    fn sample_report() {
        let commits = vec![fake_commit()];
        let hbs = super::output::from_hbs(None).unwrap();
        let config = super::config::from_yml(None).unwrap();
        let report = super::report::generate(&config, &commits);
        assert_eq!(report.scopes.len(), 2);
        assert_eq!(report.commits.len(), 1);
        assert_eq!(report.scopes[0].categories[0].title, "Features");
        assert_eq!(report.scopes[0].categories[1].title, "Notes");
        assert_eq!(report.scopes[1].categories[0].title, "Breaking Changes");

        let text = super::output::render(&report, &hbs);
        let expected = include_str!("assets/sample.md").trim();
        assert_eq!(expected, text);
    }
}
