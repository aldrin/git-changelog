// Copyright 2017 Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

#[cfg(test)]
#[test]
fn default_configuration_is_valid() {
    let config = super::config::from(&None);
    assert!(config.is_ok());
}

#[test]
fn git_last_tag() {
    match super::git::last_tag().ok() {
        Some(_) => (),
        _ => assert!(false, "unexpected error"),
    };
}

#[test]
fn commit_parse_summary() {
    use super::commit::{parse_subject, parse_number};

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
    use super::commit::parse_line;

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

#[cfg(test)]
fn fake_commit() -> super::commit::Commit {
    let header = vec![
        "2e51cdb3ef163acd31ad0ae9d1b861d544f8162b",
        "aaaaaa a a'aaaaa",
        "Sun, 22 Oct 2017 17:26:56 -0400",
    ];
    let message = include_str!("../resources/sample-commit.message").lines();

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
fn parse_commit() {
    let change = fake_commit();
    assert_eq!(&change.sha, "2e51cdb3ef163acd31ad0ae9d1b861d544f8162b");
    assert_eq!(&change.author, "aaaaaa a a'aaaaa");
    assert!(&change.time.starts_with("2017-10-22"));
    assert_eq!(
        &change.summary,
        "Demonstrate tagging conventions for commit messages"
    );
    assert_eq!(change.number, Some(4));
    assert_eq!(change.lines.iter().filter(|l| l.scope.is_some()).count(), 1);
    assert_eq!(
        change.lines.iter().filter(|l| l.category.is_some()).count(),
        5
    );
}

#[test]
fn prepare_report() {
    use super::*;
    let commits = vec![fake_commit()];
    let config = config::from(&None).unwrap();
    let report = report::generate(&config, &commits);
    println!("{:#?}", &report);
    output::render(&config, &report);
    assert_eq!(report.commits.len(), 1);
    assert_eq!(report.scopes.len(), 2);
    assert_eq!(report.scopes[0].categories[0].title, "Features");
    assert_eq!(report.scopes[0].categories[1].title, "Notes");
    assert_eq!(report.scopes[1].categories[0].title, "Breaking Changes");
}

#[test]
fn postprocess() {
    use super::*;
    let input = String::from("Fixed JIRA-1234");
    let jira = config::PostProcessor {
        lookup: r"JIRA-(?P<t>\d+)".to_string(),
        replace: r"[JIRA-$t](https://jira.company.com/$t)".to_string(),
    };
    let mut config = config::Configuration::default();
    config.post_processors = vec![jira];

    let output = output::postprocess(&config, &input);
    assert_eq!(&output, "Fixed [JIRA-1234](https://jira.company.com/1234)");
}
