// Copyright 2017 Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

use serde_yaml;
use super::{Commit, Configuration, Tag};
use std::io::{Error, ErrorKind};

/// Initialize configuration from the given file or use the default.
pub fn from_yml(given: Option<String>) -> Result<Configuration, Error> {
    // Take the given configuration YAML or load the embedded asset
    let yml = given.unwrap_or_else(|| String::from(include_str!("assets/changelog.yml")));

    // Deserialize from the YAML string and report errors, if any.
    let mut config: Configuration = serde_yaml::from_str(&yml).map_err(|e| {
        error!("Configuration file is invalid YAML: {}", e);
        Error::from(ErrorKind::InvalidInput)
    })?;

    // If no scopes are specified
    if config.scopes.is_empty() {
        // Add the default one
        config.scopes.push(Tag::default());
    }

    // If no categories are specified
    if config.categories.is_empty() {
        // Add the default one
        config.categories.push(Tag::default());
    }

    // If no date_format is specified
    if config.date_format.is_empty() {
        // Use a sensible default
        config.date_format = "%Y-%m-%d %H:%M".to_string()
    }

    // Print the log
    debug!("{:#?}", &config);

    // All good
    Ok(config)
}

/// Get the report title for a given tag
pub fn report_title(tags: &[Tag], given: &Option<String>) -> Option<String> {
    // Get the tag keyword (or blank, if none exists)
    let keyword = given.clone().unwrap_or_default();

    // Look for all known tags
    for tag in tags {
        // If the keywords match
        if tag.keyword == keyword {
            // Return the title
            return Some(tag.title.clone());
        }
    }

    // Nothing found
    None
}

/// A commit is interesting if it has at least one line with an interesting scopes and category.
pub fn is_interesting(config: &Configuration, commit: &Commit) -> bool {
    debug!("Considering {:#?}", commit);

    // Look at each line in the commit message
    for line in &commit.lines {
        // Ignore blank lines
        if line.text.is_none() {
            continue;
        }

        // Ignore lines with scopes that aren't meant to show up in the report
        if report_title(&config.scopes, &line.scope).is_none() {
            continue;
        }

        // Ignore lines with categories that aren't meant to show up in the report
        if report_title(&config.categories, &line.category).is_none() {
            continue;
        }

        // Has something of interest
        return true;
    }

    // If we get here, we found nothing of interest in this commit
    info!("Commit {} is not interesting", commit.summary);

    // Boring.
    false
}

#[cfg(test)]
mod tests {

    #[test]
    fn from_yml() {
        use super::from_yml;
        let none = None;
        let blank = Some(String::new());
        let project = Some(String::from(include_str!("../.changelog.yml")));
        let no_category = Some(String::from(r#"scopes: [{keyword:"a", title: "A"}]"#));
        let no_scope = Some(String::from(r#"categories: [{keyword:"a", title: "A"}]"#));

        assert!(from_yml(none).is_ok());
        assert!(from_yml(blank).is_err());
        assert!(from_yml(project).is_ok());
        assert!(from_yml(no_scope).is_ok());
        assert!(from_yml(no_category).is_ok());
    }

    #[test]
    fn is_interesting() {
        let blank = super::Commit::default();
        let config = super::from_yml(None).unwrap();
        assert!(!super::is_interesting(&config, &blank));
    }
}
