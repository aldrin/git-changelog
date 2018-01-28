// Copyright 2017-2018 by Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

// All git interactions
use super::Result;
use std::iter::FromIterator;
use std::process::{Command, Output};

/// Check if we're in an git repository?
pub fn in_git_repository() -> Result<bool> {
    git(&["rev-parse", "--is-inside-work-tree"]).map(|o| o.status.success())
}

/// Get the last tag
pub fn last_tag() -> Result<Option<String>> {
    last_tags(1).map(|mut v| v.pop())
}

/// Get the SHAs for all commits in the revision range
pub fn commits_in_range(range: &str) -> Result<Vec<String>> {
    git(&["log", "--format=format:%H", range]).map(|o| read_lines(&o))
}

/// Get the commit message for the given sha
pub fn get_commit_message(sha: &str) -> Result<Vec<String>> {
    git(&[
        "log",
        "--format=format:%H%n%an%n%aD%n%s%n%b",
        "--max-count=1",
        sha,
    ]).map(|o| read_lines(&o))
}

/// Get the fetch url for the given origin
pub fn get_remote_url(name: &str) -> Result<Option<String>> {
    git(&["remote", "get-url", name])
        .map(|o| read_lines(&o))
        .map(|mut v: Vec<String>| v.pop().and_then(usable_url))
}

/// Check if the remote URL is usable for links
fn usable_url(raw: String) -> Option<String> {
    if raw.to_lowercase().starts_with("http") {
        if let Some(index) = raw.rfind(".git") {
            return Some(raw[0..index].to_string());
        } else {
            return Some(raw);
        }
    }
    None
}

/// Get the last n tags
fn last_tags(n: i32) -> Result<Vec<String>> {
    git(&[
        "for-each-ref",
        &format!("--count={}", n),
        "--sort=-taggerdate",
        "--format=%(refname:short)",
        "refs/tags/*",
    ]).map(|o| read_lines(&o))
}

/// Invoke a git command with the given arguments.
fn git(args: &[&str]) -> Result<Output> {
    trace!("git {}", args.join(" "));
    let output = Command::new("git").args(args).output()?;
    if output.status.success() {
        Ok(output)
    } else {
        Err(format_err!("{}", String::from_utf8_lossy(&output.stderr)))
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
        assert!(super::in_git_repository().unwrap());
    }

    #[test]
    fn last_tag() {
        assert!(super::last_tag().is_ok());
    }

    #[test]
    fn commits_in_range() {
        use super::commits_in_range;
        let commits = commits_in_range("v0.1.1..v0.2.0");
        assert!(commits.is_ok(), "{:?}", commits);
        assert_eq!(commits.unwrap().len(), 2);
    }

    #[test]
    fn get_commit_message() {
        use super::get_commit_message;
        assert!(get_commit_message("v0.1.1").is_ok());
        assert!(get_commit_message("bad").is_err());
    }

    #[test]
    fn get_usable_url() {
        use super::usable_url;
        let ssh = String::from("git@github.com:aldrin/git-changelog.git");
        let raw = String::from("https://github.com/aldrin/git-changelog.git");
        let usable = "https://github.com/aldrin/git-changelog";
        assert_eq!(usable_url(usable.to_string()), Some(usable.to_string()));
        assert_eq!(usable_url(raw), Some(usable.to_string()));
        assert_eq!(usable_url(ssh), None);
    }

    #[test]
    fn get_remote_url() {
        use super::get_remote_url;
        let expected = Some(String::from("https://github.com/aldrin/git-changelog"));
        let found = get_remote_url("origin").unwrap();
        assert!(get_remote_url("bad").is_err());
        assert_eq!(found, expected);
    }
}
