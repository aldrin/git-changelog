extern crate changelog;
extern crate difference;
extern crate env_logger;
extern crate log;
extern crate regex;

use changelog::*;
use regex::Regex;

#[test]
#[cfg(feature = "handlebars")]
fn readme_example() {
    let config = builtin_config();
    let commits = vec![readme_commit()];
    let mut log = ChangeLog::from(commits.into_iter(), &config);
    log.range = String::from("1d82af9^..1d82af9");
    let md = render(&log, &config.output).unwrap();
    let expected = include_str!("../src/assets/sample.md");

    // Remove the repo URL so this passes when checking out from a different repo
    let url_regex = Regex::new("https://github.com/[^/]+/git-changelog").unwrap();
    let expected = url_regex.replace_all(expected, "<repo url>");
    let md = url_regex.replace_all(&md, "<repo url>");

    let diff = difference::Changeset::new(&expected, &md, " ");
    assert_eq!(diff.diffs.len(), 1, "{:#?}", diff.diffs);
}

#[test]
fn library_example() {
    // Create a custom configuration
    let mut config = Configuration::new();

    // Pick the category or scope keywords that match your project conventions
    config
        .conventions
        .categories
        .push(Keyword::new("feature", "New Features"));
    config
        .conventions
        .categories
        .push(Keyword::new("break", "Breaking Changes"));

    // Pick the range of commits
    let range = "v0.1.1..v0.2.0";

    // Generate a changelog for the range with the configuration
    let changelog = ChangeLog::from_range(range, &config);

    // Print
    println!("{}", changelog);
    println!("{:#?}", changelog);
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
