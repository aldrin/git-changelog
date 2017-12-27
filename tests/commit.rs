// Copyright 2017 Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

extern crate changelog;

mod common;
use changelog::commit;

#[test]
fn commit_parse_summary() {
    use commit::{parse_number, parse_subject};

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

    let line = parse_line("-fix: foo bar");
    assert_eq!(None, line.scope);
    assert_eq!(Some(String::from("fix")), line.category);
    assert_eq!(Some(String::from(" foo bar")), line.text);

    let line = parse_line("-breaK(SHELL): foo bar");
    assert_eq!(Some(String::from("shell")), line.scope);
    assert_eq!(Some(String::from("break")), line.category);
    assert_eq!(Some(String::from(" foo bar")), line.text);

    let line = parse_line("- foo bar");
    assert_eq!(None, line.scope);
    assert_eq!(None, line.category);
    assert_eq!(Some(String::from(" foo bar")), line.text);

    let line = parse_line("foo bar");
    assert_eq!(None, line.scope);
    assert_eq!(None, line.category);
    assert_eq!(Some(String::from("foo bar")), line.text);
}

#[test]
fn parse_commit() {
    let change = common::fake_commit();
    assert_eq!(&change.sha, "2e51cdb3ef163acd31ad0ae9d1b861d544f8162b");
    assert_eq!(&change.author, "aaaaaa a a'aaaaa");
    assert!(&change.time.starts_with("2017-10-22"));
    assert_eq!(
        &change.summary,
        "Demonstrate tagging conventions for commit messages"
    );
    assert_eq!(change.number, Some(5));
    assert_eq!(change.lines.iter().filter(|l| l.scope.is_some()).count(), 1);
    assert_eq!(
        change.lines.iter().filter(|l| l.category.is_some()).count(),
        5
    );
}
