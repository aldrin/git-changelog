// Copyright 2017-2018 by Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

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
