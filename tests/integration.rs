extern crate changelog;
extern crate difference;
extern crate env_logger;
extern crate log;

use changelog::*;

#[test]
fn readme_example() {
    let config = builtin_config();
    let commits = vec![readme_commit()];
    let mut log = ChangeLog::from(commits.into_iter(), &config);
    log.range = String::from("1d82af9^..1d82af9");
    let md = render(&log, &config.output).unwrap();
    let expected = include_str!("../src/assets/sample.md");
    let diff = difference::Changeset::new(expected, &md, " ");
    assert_eq!(diff.diffs.len(), 1, "{:#?}", diff.diffs);
}

fn builtin_config() -> Configuration {
    Configuration::from_yaml(include_str!("../src/assets/changelog.yml")).unwrap()
}

fn readme_commit() -> Commit {
    let mut commit = vec![
        "1d82af9a1bd05c100b7b50bdcda3db39a5cddcdf",
        "aaaaaa a a'aaaaa",
        "Sun, 22 Oct 2017 17:26:56 -0400",
    ];
    commit.extend(include_str!("../src/assets/sample-commit.message").lines());
    let commit: Vec<String> = commit.into_iter().map(str::to_string).collect();
    Commit::from_lines(commit)
}
