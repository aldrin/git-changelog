// Copyright 2017-2018 by Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>
// Commit fetch and parsing logic
use git;
use std::{fmt, str};
use nom::{is_alphanumeric, IResult};

/// A single commit
#[derive(Debug, Default, Serialize, Eq, PartialEq)]
pub struct Commit {
    /// The SHA
    pub sha: String,

    /// The author
    pub author: String,

    /// The timestamp
    pub time: String,

    /// The summary
    pub summary: String,

    /// The change number
    pub number: Option<u32>,

    /// The message
    pub message: String,
}

/// A list of commit revisions
pub struct CommitList {
    /// The log command
    input: String,

    /// The commits in the log
    commits: Vec<String>,
}

/// The commit message
pub struct CommitMessage<'a>(Vec<&'a str>);

/// A single line in a commit change message
#[derive(Default, Debug)]
pub struct Line {
    /// The scope
    pub scope: Option<String>,

    /// The category
    pub category: Option<String>,

    /// The text
    pub text: Option<String>,
}

impl<T: AsRef<str>> From<T> for Commit {
    /// Construct a commit from the revision
    fn from(input: T) -> Self {
        let revision = input.as_ref();
        match git::get_commit_message(revision) {
            Ok(lines) => Commit::from_lines(lines),
            Err(why) => {
                error!("Commit {} will be skipped (Reason: {})", revision, why);
                Commit::default()
            }
        }
    }
}

impl Commit {
    pub fn from_lines(mut lines: Vec<String>) -> Self {
        let mut commit = Self::default();

        commit.sha = lines.remove(0);
        commit.author = lines.remove(0);
        commit.time = lines.remove(0);

        let subject = lines.remove(0);
        commit.number = parse_number(&subject);
        commit.summary = parse_subject(&subject);
        commit.message = lines.join("\n");

        commit
    }
}

impl<'a> From<&'a str> for CommitList {
    /// Convenience constructor from a simple range
    fn from(range: &str) -> Self {
        Self::from(vec![range.to_string()])
    }
}

impl From<Vec<String>> for CommitList {
    /// Generate a commit list from the list of strings, interpreting them as `git log` arguments.
    fn from(git_log_args: Vec<String>) -> Self {
        // Record the log input
        let input = git_log_args.join(" ");

        // Get the commits that `git log` would have returned
        let commits = match git::commits_in_log(&git_log_args) {
            Ok(commits) => commits,
            Err(why) => {
                error!("Invalid log input {} (Reason: {})", input, why);
                vec![]
            }
        };
        CommitList { commits, input }
    }
}

impl Iterator for CommitList {
    type Item = Commit;
    fn next(&mut self) -> Option<Self::Item> {
        self.commits.pop().map(Commit::from)
    }
}

impl<'a> IntoIterator for &'a Commit {
    type Item = Line;
    type IntoIter = CommitMessage<'a>;
    fn into_iter(self) -> Self::IntoIter {
        CommitMessage(self.message.lines().collect())
    }
}

impl<'a> Iterator for CommitMessage<'a> {
    type Item = Line;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            Some(parse_line(self.0.remove(0)))
        }
    }
}

impl fmt::Display for Commit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.sha, self.summary)
    }
}

impl fmt::Display for CommitList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({} commits)", self.input, self.commits.len())
    }
}

/// Parse the commit subject removing the numbers tags.
fn parse_subject(line: &str) -> String {
    // Find the first number opener on the commit subject
    let first_open = line.find("(#").unwrap_or_else(|| line.len());

    // Everything up to the first number opener is the subject
    String::from(line.get(0..first_open).unwrap_or_else(|| line).trim())
}

/// Parse the commit number
fn parse_number(line: &str) -> Option<u32> {
    // The commit number is the last number on the subject
    let last_open = line.rfind("(#");

    // The last number opener
    let last_close = line.rfind(')');

    // If no number found on subject line
    if last_open.is_none() || last_close.is_none() {
        // The commit has no number
        None
    } else {
        // Extract the bounds of the last number
        let end = last_close.unwrap();
        let start = last_open.unwrap() + "(#".len();

        // Parse it to a number
        let num = line.get(start..end).map(|s| s.parse().ok());

        // If valid, we have a number
        num.unwrap_or(None)
    }
}

/// Parse an individual message line
fn parse_line(line: &str) -> Line {
    // Parse the tags in the line
    match tagged_change(line) {
        // If parser succeeded, we have our line
        IResult::Done(_, l) => l,

        // If parser fails, draw a blank
        _ => Line::default(),
    }
}

/// A change message line is one of the following types
named!(tagged_change<&str, Line>,
       alt!(with_category
           | with_category_scope
           | with_category_text
           | with_category_scope_text
           | with_text
       ));

/// A line that has just a simple change (no tags).
named!(with_text<&str, Line>,
       do_parse!(opt!(tag!("-")) >>
                 text: whatever >>
                 (Line{
                     scope: None,
                     category: None,
                     text: Some(text)
                 })));

/// A line that has just a category
named!(with_category<&str, Line>,
       do_parse!(
           tag!("-") >> category: tagname >>
               tag!(":") >> eof!() >>
               (Line{
                   scope: None,
                   category: Some(category),
                   text: None
               })));

/// A line that has just a category and scope, but no change text
named!(with_category_scope<&str, Line>,
       do_parse!(
           tag!("-") >> category: tagname >>
               tag!("(") >> scope: tagname >>
               tag!("):") >> eof!() >>
               (Line{
                   scope: Some(scope),
                   category: Some(category),
                   text: None
               })));

/// A line that has a category and a change text, but no scope.
named!(with_category_text<&str, Line>,
       do_parse!(
           tag!("-") >> category: tagname >>
               tag!(":") >> text: whatever >>
               (Line{
                   scope: None,
                   category: Some(category),
                   text: Some(text)
               })));

/// A line that has everything, i.e. category, scope and a change.
named!(with_category_scope_text<&str, Line>,
       do_parse!(
           tag!("-") >> category: tagname >>
               tag!("(") >> scope: tagname >>
               tag!("):") >> text: whatever >>
               (Line{
                   scope: Some(scope),
                   category: Some(category),
                   text: Some(text)
                })));

/// Consume whatever is left and return a String
named!(whatever<&str, String>,
       map!(take_while1_s!(|_| true), String::from));

/// Consume an acceptable tag name and return a String
named!(tagname<&str, String>,
       map!(ws!(take_while1_s!(|c| is_alphanumeric(c as u8))), str::to_lowercase));

#[cfg(test)]
mod tests {
    #[test]
    fn commit_fetch() {
        use super::{Commit, CommitList};
        let head = Commit::from("2c5dda2e");
        let list = CommitList::from("2c5dda2e^..2c5dda2e");
        assert_eq!(list.to_string(), "2c5dda2e^..2c5dda2e (1 commits)");
        let also_head = list.into_iter().next().unwrap();
        assert_eq!(head.sha, also_head.sha);
        assert!(head.to_string().starts_with("2c5dda2e"));
    }

    #[test]
    fn negative() {
        assert!(super::Commit::from("no-such-commit").summary.is_empty());
        assert_eq!(super::CommitList::from("bad-range").into_iter().count(), 0);
    }

    #[test]
    fn commit_lines() {
        use super::Commit;
        let reference = &Commit::from("2c5dda2e5ec6d0ad7abdcd20661bf2cb846ee5f2");
        assert_eq!(reference.into_iter().count(), 17);
    }

    #[test]
    fn commit_parse_summary() {
        use super::{parse_number, parse_subject};

        // most common - simple PR merge
        let message = "foo bar (#123)";
        assert_eq!(parse_subject(message), "foo bar");
        assert_eq!(parse_number(message), Some(123));

        // not a PR merge
        let message = "foo bar ()()";
        assert_eq!(parse_subject(message), message);
        assert_eq!(parse_number(message), None);

        // cherry-picked multi-PR commits
        let message = "foo bar #123 (#101)(#103)";
        assert_eq!(parse_subject(message), "foo bar #123");
        assert_eq!(parse_number(message), Some(103));
    }

    #[test]
    fn commit_parse_line() {
        use commit::parse_line;

        let line = parse_line("- break(shell): foo bar");
        assert_eq!(Some(String::from("shell")), line.scope);
        assert_eq!(Some(String::from("break")), line.category);
        assert_eq!(Some(String::from(" foo bar")), line.text);

        let line = parse_line("-BREAK ( Shell ): foo bar");
        assert_eq!(Some(String::from("shell")), line.scope);
        assert_eq!(Some(String::from("break")), line.category);
        assert_eq!(Some(String::from(" foo bar")), line.text);

        let line = parse_line("- break(shell):");
        assert_eq!(Some(String::from("shell")), line.scope);
        assert_eq!(Some(String::from("break")), line.category);
        assert_eq!(None, line.text);

        let line = parse_line("- break ( SHELL ):");
        assert_eq!(Some(String::from("shell")), line.scope);
        assert_eq!(Some(String::from("break")), line.category);
        assert_eq!(None, line.text);

        let line = parse_line("-fix:");
        assert_eq!(None, line.scope);
        assert_eq!(Some(String::from("fix")), line.category);
        assert_eq!(None, line.text);

        let line = parse_line("- fix: foo bar");
        assert_eq!(None, line.scope);
        assert_eq!(Some(String::from("fix")), line.category);
        assert_eq!(Some(String::from(" foo bar")), line.text);

        let line = parse_line("- FIX  : foo bar");
        assert_eq!(None, line.scope);
        assert_eq!(Some(String::from("fix")), line.category);
        assert_eq!(Some(String::from(" foo bar")), line.text);

        let line = parse_line("- foo bar");
        assert_eq!(None, line.scope);
        assert_eq!(None, line.category);
        assert_eq!(Some(String::from(" foo bar")), line.text);

        let line = parse_line("foo bar");
        assert_eq!(None, line.scope);
        assert_eq!(None, line.category);
        assert_eq!(Some(String::from("foo bar")), line.text);

        let line = parse_line("");
        assert_eq!(None, line.text);
        assert_eq!(None, line.scope);
        assert_eq!(None, line.category);
    }
}
