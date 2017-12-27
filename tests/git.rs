extern crate changelog;
use changelog::git;

#[test]
fn git_last_tag() {
    assert!(git::last_tag().is_ok());
}

#[test]
fn git_commit_in_range() {
    let range = vec![
        String::from("v0.1.1"),
        String::from("..."),
        String::from("v0.2.0"),
    ];
    let commits = git::commits_in_range(&range).unwrap();
    assert_eq!(commits.len(), 2);
}

#[test]
fn git_commit_messages() {
    assert!(git::get_commit_message("v0.1.1").is_ok());
    assert!(git::get_commit_message("bad").is_err());
}

#[test]
fn in_git_dir() {
    assert!(git::in_git_repository().is_ok());
}
