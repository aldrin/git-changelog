// Copyright 2017 Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>
extern crate changelog;
extern crate env_logger;
extern crate log;

#[test]
fn generate_changelog() {
    use std::env::current_dir;
    use env_logger::LogBuilder;
    use log::LogLevelFilter;

    let mut builder = LogBuilder::new();
    builder.filter(Some("changelog"), LogLevelFilter::Info);
    builder.init().unwrap();

    let sha = "1d82af9a1bd05c100b7b50bdcda3db39a5cddcdf";
    let range = format!("{}...{}^", sha, sha);
    let dir = current_dir().unwrap();

    // Choice 1 -- generate using template
    let mut input = changelog::Input::default();
    input.revision_range = Some(vec![range.clone()]);
    input.config_file = Some(String::from("src/assets/changelog.yml"));
    input.output_template_file = Some(String::from("src/assets/changelog.hbs"));
    let output = changelog::run(input, &dir);
    assert!(output.is_ok());
    println!("{}", output.unwrap());

    // Choice 2 -- generate as JSON
    let mut input = changelog::Input::default();
    input.output_json = true;
    input.revision_range = Some(vec![range.clone()]);
    input.config_file = Some(String::from("src/assets/changelog.yml"));
    let output = changelog::run(input, &dir);
    assert!(output.is_ok());
    println!("{}", output.unwrap());
}
