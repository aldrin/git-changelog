// Copyright 2017 Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>
extern crate changelog;
extern crate tempdir;

#[test]
fn default_configuration_is_valid() {
    assert!(changelog::config::from(&None).is_ok());
}

#[test]
fn bad_files_are_reported() {
    assert!(changelog::config::from(&Some("unknown_file".to_string())).is_err());
    assert!(changelog::config::from(&Some("Cargo.toml".to_string())).is_err());
}

#[test]
fn config_files_are_discovered() {
    use std::env::current_dir;
    let discovered = changelog::config::find_file(current_dir().ok()).unwrap();
    assert!(discovered.ends_with(changelog::config::FILE));
    assert!(changelog::config::from(&Some(discovered)).is_ok());
}

#[test]
fn config_files_are_optional() {
    use tempdir::TempDir;
    let testdir = TempDir::new("git-changelog-test").unwrap().into_path();
    assert!(changelog::config::find_file(Some(testdir)).is_none());
}
