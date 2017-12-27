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
extern crate serde_yaml;

pub mod git;
pub mod tool;
pub mod commit;
pub mod config;
pub mod report;
pub mod output;
