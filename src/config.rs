// Copyright 2017 Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

use serde_yaml;
use std::fs::File;
use commit::Commit;
use std::path::PathBuf;
use handlebars::Handlebars;
use std::io::{BufReader, Error, ErrorKind, Read};

/// The default configuration file name
pub const FILE: &str = ".changelog.yml";

/// A tag definition
#[serde(default)]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Tag {
    /// The identifying keyword
    pub keyword: String,

    /// The report heading
    pub title: String,
}

/// A post-processor definition
#[serde(default)]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PostProcessor {
    /// The lookup pattern
    pub lookup: String,

    /// The replace pattern
    pub replace: String,
}

/// The tool configuration structure (can be specified in a file)
#[serde(default)]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Configuration {
    /// The change category configuration
    pub categories: Vec<Tag>,

    /// The change scope configuration
    pub scopes: Vec<Tag>,

    /// The report title
    pub title: String,

    /// The report template file
    pub template: String,

    /// The date format
    pub date_format: String,

    /// The line post-processors
    pub post_processors: Vec<PostProcessor>,
}

/// Initialize configuration from the given file or use the default.
pub fn from(filename: &Option<String>) -> Result<Configuration, Error> {
    // Take the given filename
    let mut config: Configuration = match *filename {
        // If none is given, initialize from the embedded default
        None => {
            info!("Using default built-in configuration");
            serde_yaml::from_str(include_str!("../changelog.yml"))
        }

        // If some file is given, read it and deserialize into the config structure
        Some(ref file) => {
            info!("Using configuration from {}", file);
            serde_yaml::from_reader(File::open(file)?)
        }
    }.map_err(|e| {
        // Inform and get out
        error!("Invalid configuration file '{:?}', {}.", filename, e);
        Error::from(ErrorKind::InvalidInput)
    })?;

    // Read the template in the effective configuration
    let template = if config.template.is_empty() {
        // If empty, initialize from the embedded default
        String::from(include_str!("../changelog.hbs"))
    } else {
        // Have a file name, read it fully
        let mut template = String::new();
        let file = File::open(config.template)?;
        BufReader::new(file).read_to_string(&mut template)?;

        // And use as the template
        template
    };

    // Render the template and check for syntax issues early
    Handlebars::new()
        .register_template_string("t", &template)
        .map_err(|e| {
            // Syntax error, inform and get out
            error!("Invalid handlebar template: '{}'", e);
            Error::from(ErrorKind::InvalidInput)
        })?;

    // Overwrite the template we'll use.
    config.template = template;

    // If no data_format is specified
    if config.date_format.is_empty() {
        // Use a sensible default
        config.date_format = "%Y-%m-%d %H:%M".to_string()
    }

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

    // Print the log
    debug!("CONFIG: {:#?}", &config);

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

/// Identify the closest configuration file that should be used for this run
pub fn find_file(start_dir: Option<PathBuf>) -> Option<String> {
    // Start at the current directory
    if let Some(mut cwd) = start_dir {
        // While we have hope
        while cwd.exists() {
            // Set the filename we're looking for
            cwd.push(FILE);

            // If we find it
            if cwd.is_file() {
                // return it
                return Some(cwd.to_string_lossy().to_string());
            }

            // If not, remove the filename
            cwd.pop();

            // If we have room to go up
            if cwd.parent().is_some() {
                // Go up the path
                cwd.pop();
            } else {
                // Get out
                break;
            }
        }
    }

    // No file found
    None
}
