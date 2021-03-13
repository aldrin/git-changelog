// Copyright 2017-2018 by Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>
//! A generator for categorized change logs.
//!
//! This crate generates change logs (a.k.a release notes) that are typically distributed at project
//! release milestones. Unlike other tools that do the same, this one does not require you to follow
//! any particular git workflow conventions. All it assumes is that you'll pick a few *keywords* (or
//! use the built-in ones) to annotate lines in your commit messages.
//!
//! When you wish to record a user visible change (e.g. new feature, bug fix, breaking change, etc.)
//! you write a normal commit message and annotate some lines in it with your chosen keywords. The
//! annotated lines are used at report generation time to organize changes into *categories* and
//! *scopes*. The organized changes are then rendered as a pretty and accurate change log. Commit
//! messages without tags are quietly ignored and you are free to tag as little or as much as you
//! want.
//!
//! See [README] for details.
//!
//! # Usage
//!
//! Install the executable wrapper as follows:
//!
//! ```bash
//! $ cargo install git-changelog
//! ```
//!
//! Once on `PATH`, the tool works like a normal git sub-command (e.g. `git log`) and takes a
//! [revision range] as input. It looks at all commits in the range and uses the keywords it finds
//! in their messages to generate the report. Simple. 🙂
//!
//! ```bash
//! $ git changelog v0.1.1..v0.2.0
//! ```
//!
//! If you don't provide a revision range, `<last-tag>..HEAD` is used. If no tags are defined, just
//! the last commit is picked.
//!
//! ```bash
//! $ git changelog -d
//! INFO: Reading file '/Users/aldrin/Code/git-changelog/.changelog.yml'
//! INFO: Using revision range 'v0.2.0..HEAD (15 commits)'
//! ...
//! ```
//!
//! Note that using `-d` can you give you some insight into the tool operations. For more complex range
//! selections you can use `git log` arguments as shown below:
//!
//! ```bash
//! $ git changelog -- --author aldrin --reverse --since "1 month ago"
//! ```
//!
//! Note the `--` before you start the `git log` arguments.
//!
//! # Library Usage
//!
//! The crate functionality is also usable as a library as shown below:
//!
//! ```toml
//! [dependencies]
//! git-changelog = "0.3"
//! ```
//!
//! ```rust
//! extern crate changelog;
//! println!("{}", changelog::ChangeLog::new());
//! ```
//!
//! The configuration used in simple example shown above is picked from the repository configuration
//! file (if one is found) or from built-in defaults. Advanced usage can programmatically manage
//! these as follows:
//!
//! ```rust
//! use changelog::{Configuration, Keyword, ChangeLog};
//!
//! // Create a custom configuration
//! let mut config = Configuration::new();
//!
//! // Pick the category or scope keywords that match your project conventions
//! config.conventions.categories.push(Keyword::new("feature", "New Features"));
//! config.conventions.categories.push(Keyword::new("break", "Breaking Changes"));
//!
//! // Pick the range of commits
//! let range = "v0.1.1..v0.2.0";
//!
//! // Generate a changelog for the range with the configuration
//! let changelog = ChangeLog::from_range(range, &config);
//!
//! // Pick output preferences
//! config.output.json = true;
//!
//! // Render
//! assert!(changelog::render(&changelog, &config.output).is_ok());
//! ```
//! [revision range]: https://git-scm.com/book/en/v2/Git-Tools-Revision-Selection#double_dot
//! [README]: https://github.com/aldrin/git-changelog/blob/master/README.md

extern crate chrono;
#[macro_use]
extern crate anyhow;
#[cfg(feature = "handlebars")]
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

mod changelog;
mod commit;
mod git;
mod input;
mod output;
#[cfg(feature = "handlebars")]
mod template_hbs;

pub use changelog::Category;
pub use changelog::ChangeLog;
pub use changelog::Scope;
pub use commit::Commit;
pub use commit::CommitList;
pub use commit::CommitMessage;
pub use git::in_git_repository;
pub use input::Configuration;
pub use input::Conventions;
pub use input::Keyword;
pub use input::OutputPreferences;
pub use input::PostProcessor;
pub use input::CONFIG_FILE;
pub use input::TEMPLATE_FILE;
pub use output::render;

pub type Result<T> = std::result::Result<T, anyhow::Error>;
