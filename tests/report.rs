// Copyright 2017 Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

extern crate changelog;

mod common;
use changelog::config;
use changelog::report;
use changelog::output;

use std::fs::File;
use std::io::prelude::*;

#[test]
fn prepare_report() {
    let commits = vec![common::fake_commit()];
    let config = config::from(&None).unwrap();
    let report = report::generate(&config, &commits);
    let text = output::render(&config, &report);
    assert_eq!(report.commits.len(), 1);
    assert_eq!(report.scopes.len(), 2);
    assert_eq!(report.scopes[0].categories[0].title, "Features");
    assert_eq!(report.scopes[0].categories[1].title, "Notes");
    assert_eq!(report.scopes[1].categories[0].title, "Breaking Changes");
    
    let mut file = File::create("tests/common/sample.md").unwrap();
    file.write_all(text.as_bytes()).unwrap();
}

#[test]
fn postprocess() {
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
