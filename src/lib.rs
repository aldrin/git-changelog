// Copyright 2017-2018 by Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>
//! A generator for categorized change logs.
//!
//! This crate generates change logs (a.k.a release notes) that are typically distributed at project release
//! milestones. It looks for keywords in git commit messages and uses them to produce a presentable and categorized
//! change log.
//!
//! The associated crate [`git-changelog`] provides an executable wrapper around this crate.
//!
//! # Use
//!
//! The crate functionality is also usable as a library as shown below:
//!
//! ```rust
//! extern crate changelog;
//! println!("{}", changelog::ChangeLog::new());
//! ```
//!
//! The configuration used in simple example shown above is picked from the repository configuration file (if one is
//! found) or from built-in defaults. Advanced usage can programmatically manage these as follows:
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
//! let range = String::from("v0.1.1..v0.2.0");
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
//!
//! [`git-changelog`]: ../git_changelog/index.html

extern crate chrono;
#[macro_use]
extern crate failure;
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
mod input;
mod commit;
mod output;
mod changelog;

pub use git::in_git_repository;
pub use input::Configuration;
pub use input::Conventions;
pub use input::OutputPreferences;
pub use input::Keyword;
pub use input::PostProcessor;
pub use input::TEMPLATE_FILE;
pub use input::CONFIG_FILE;
pub use commit::Commit;
pub use commit::CommitList;
pub use commit::CommitMessage;
pub use changelog::ChangeLog;
pub use changelog::Category;
pub use changelog::Scope;
pub use output::render;

pub type Result<T> = std::result::Result<T, failure::Error>;
