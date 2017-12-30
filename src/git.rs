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
    git(&[
        "log",
        "--format=format:%H%n%an%n%aD%n%s%n%b",
        "--max-count=1",
        sha,
    ]).map(|o| read_lines(&o))
}

/// Get the last n tags
fn last_tags(n: i32) -> Result<Vec<String>, Error> {
    git(&[
        "for-each-ref",
        &format!("--count={}", n),
        "--sort=-taggerdate",
        "--format=%(refname:short)",
        "refs/tags/*",
    ]).map(|o| read_lines(&o))
}

/// Invoke a git command with the given arguments.
fn git(args: &[&str]) -> Result<Output, Error> {
    debug!("git {}", args.join(" "));
    let output = Command::new("git").args(args).output()?;
    if output.status.success() {
        Ok(output)
    } else {
        println!("{}", String::from_utf8_lossy(&output.stderr));
        println!("{}", String::from_utf8_lossy(&output.stdout));
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

#[cfg(test)]
mod tests {
    #[test]
    fn in_git_repository() {
        assert!(super::in_git_repository().is_ok());
    }

    #[test]
    fn last_tag() {
        assert!(super::last_tag().is_ok());
    }

    #[test]
    fn commits_in_range() {
        use super::commits_in_range;
        let range = vec![String::from("v0.1.1...v0.2.0")];
        let commits = commits_in_range(&range);
        assert!(commits.is_ok(), "{:?}", commits);
        assert_eq!(commits.unwrap().len(), 2);
    }

    #[test]
    fn get_commit_message() {
        use super::get_commit_message;
        assert!(get_commit_message("v0.1.1").is_ok());
        assert!(get_commit_message("bad").is_err());
    }
}
