// Copyright 2017 Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

/// Git commands
use std::iter::FromIterator;
use std::io::{Error, ErrorKind};
use std::process::{Command, Output};

/// Check if we're in an git repository?
pub fn in_git_repository() -> Result<Output, Error> {
    git(&["rev-parse", "--is-inside-work-tree"])
}

/// Get the last tag
pub fn last_tag() -> Result<Option<String>, Error> {
    last_tags(1).map(|mut v| v.pop())
}

/// Get the SHAs for all commits in the revision range
pub fn commits_in_range(range: &[String]) -> Result<Vec<String>, Error> {
    let mut log = vec!["log", "--format=format:%H"];
    for r in range {
        log.push(r)
    }
    git(&log).map(|o| read_lines(&o))
}

/// Get the commit message for the given sha
pub fn get_commit_message(sha: &str) -> Result<Vec<String>, Error> {
    git(
        &[
            "log",
            "--format=format:%H%n%an%n%aD%n%s%n%b",
            "--max-count=1",
            sha,
        ],
    ).map(|o| read_lines(&o))
}

/// Get the last n tags
fn last_tags(n: i32) -> Result<Vec<String>, Error> {
    git(
        &[
            "for-each-ref",
            &format!("--count={}", n),
            "--sort=-taggerdate",
            "--format=%(refname:lstrip=2)",
            "refs/tags/*",
        ],
    ).map(|o| read_lines(&o))
}

/// Invoke a git command with the given arguments.
fn git(args: &[&str]) -> Result<Output, Error> {
    debug!("git {}", args.join(" "));
    let output = Command::new("git").args(args).output()?;
    if output.status.success() {
        Ok(output)
    } else {
        error!("{}", String::from_utf8_lossy(&output.stderr));
        Err(Error::from(ErrorKind::InvalidData))
    }
}

/// Read the lines from the output and gather them into a String collection.
fn read_lines<T: FromIterator<String>>(o: &Output) -> T {
    String::from_utf8_lossy(&o.stdout)
        .lines()
        .map(String::from)
        .collect::<T>()
}
