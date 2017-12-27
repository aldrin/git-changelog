// Copyright 2017 Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>
extern crate changelog;
extern crate tempdir;

use changelog::commit;

pub fn fake_commit() -> commit::Commit {
    let header = vec![
        "2e51cdb3ef163acd31ad0ae9d1b861d544f8162b",
        "aaaaaa a a'aaaaa",
        "Sun, 22 Oct 2017 17:26:56 -0400",
    ];
    let message = include_str!("sample-commit.message").lines();

    let mut lines: Vec<String> = Vec::new();
    for l in header {
        lines.push(l.to_string());
    }
    for l in message {
        lines.push(l.to_string());
    }
    commit::parse(&lines, "%Y-%m-%d %H:%M")
}
