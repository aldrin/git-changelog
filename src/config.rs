// Copyright 2017 Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

use serde_yaml;
use std::fs::File;
use commit::{Commit, Line};
use handlebars::Handlebars;
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Read, BufReader};

/// The tool configuration structure (can be specified in a file)
#[serde(default)]
#[derive(Serialize, Deserialize, Default)]
pub struct Configuration {
    /// The change category configuration
    pub categories: TagConfiguration,
    /// The change scope configuration
    pub scopes: TagConfiguration,
    /// The report configuration
    pub report: ReportConfiguration,
}

/// Configuration of scopes and categories
#[serde(default)]
#[derive(Serialize, Deserialize, Default)]
pub struct TagConfiguration {
    /// The tags that identify the item.
    pub tags: Vec<String>,
    /// A mapping from the tag to the readable title in reports.
    pub titles: HashMap<String, String>,
    /// The default tag to use if none is specified
    pub default: String,
}

/// Report configuration
#[serde(default)]
#[derive(Serialize, Deserialize, Default)]
pub struct ReportConfiguration {
    /// The report title
    pub title: String,
    /// The report template
    pub template: String,
    /// The date format
    pub date_format: String,
}

/// Initialize configuration from the given file or the default.
pub fn from(filename: Option<&str>) -> Result<Configuration, Error> {
    let cfg = match filename {
        None => serde_yaml::from_str(include_str!("../resources/config.yml")),
        Some(file) => serde_yaml::from_reader(File::open(file)?),
    };

    let mut config: Configuration = cfg.map_err(|e| {
        error!(
            "Invalid configuration file '{}', {}.",
            filename.unwrap_or("default"),
            e
        );
        Error::from(ErrorKind::InvalidInput)
    })?;

    // Read the template
    let template = if config.report.template.is_empty() {
        String::from(include_str!("../resources/report.handlebars"))
    } else {
        let mut template = String::new();
        let file = File::open(config.report.template)?;
        BufReader::new(file).read_to_string(&mut template)?;
        template
    };

    // Render the template and check for syntax issues early
    Handlebars::new()
        .register_template_string("t", &template)
        .map_err(|e| {
            error!("Invalid handlebar template: '{}'", e);
            Error::from(ErrorKind::InvalidInput)
        })?;

    // All seems well, remember the text
    config.report.template = template;

    // If we don't have a date format
    if config.report.date_format.is_empty() {
        config.report.date_format = String::from("%Y-%m-%d");
    }

    Ok(config)
}

impl Configuration {
    /// Validate a line and replace its category and scope as configured
    pub fn validate(&self, line: &Line) -> Line {
        let text = line.text.clone();
        let scope = self.scopes.validate(line.scope.clone());
        let category = self.categories.validate(line.category.clone());
        Line {
            text,
            category,
            scope,
        }
    }

    /// A commit is interesting if it has at least one line with an interesting scopes and category.
    pub fn is_interesting(&self, commit: &Commit) -> bool {
        for line in &commit.lines {
            if line.text.is_none() {
                continue;
            }
            if self.scopes.validate(line.scope.clone()).is_none() {
                continue;
            }
            if self.categories.validate(line.category.clone()).is_none() {
                continue;
            }
            return true;
        }
        info!("Commit {} is not interesting", commit.summary);
        false
    }
}

impl TagConfiguration {
    /// Check if the given tag needs to be included into the report
    pub fn validate(&self, given: Option<String>) -> Option<String> {
        given
            // If none is given, use configured default
            .or_else(|| Some(self.default.clone()))
            // Check if the tag is on the configured tag list
            .and_then(|tag| if self.tags.iter().any(|t| t == &tag) {
                // Yes, we can use it.
                Some(tag)
            } else {
                // Unknown tag, can't use it.
                None
            })
    }

    /// Get the title of the given item
    pub fn get_title(&self, item: &str) -> String {
        match self.titles.get(item) {
            Some(text) => text,
            None => item,
        }.to_string()
    }

    /// Print the specification for visual inspection
    pub fn show(&self, item: &str, warn_if_empty: bool) {
        if self.tags.is_empty() {
            if warn_if_empty {
                warn!("No {} tags defined", item);
            }
        } else {
            info!("Available {} tags: {:?}", item, self.tags);
            if self.default.is_empty() {
                warn!("No default {} tag defined.", item);
            } else {
                info!(r#"Using "{}" as the default {} tag"#, self.default, item);
            }
        }
        for t in &self.tags {
            match self.titles.get(t) {
                None => warn!(r#"Title for {} tag "{}" is missing"#, item, t),
                Some(title) => debug!(r#"Title for {} tag "{}" is "{}""#, item, t, title),
            };
        }
    }
}
