// Copyright 2017-2018 by Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

use std::fs::File;
use super::Result;
use std::io::prelude::*;
use serde_yaml::from_str;
use std::env::current_dir;

/// The YAML configuration file name (`.changelog.yml`).
///
/// The library looks for a file with this name in the current directory and all its ancestors (up to root). If
/// one is found, it is used to initialize configuration, if not the default configuration is used.
pub const CONFIG_FILE: &str = ".changelog.yml";

/// The embedded configuration used when none is provided by the user.
const CONFIG_DEFAULT: &str = include_str!("assets/changelog.yml");

/// The Handlebars template file name (`.changelog.hbs`).
///
/// The library looks for a file with this name (`.changelog.hbs`) in the current directory and all its ancestors
/// (up to root). If one is found, it is used to render the changelog, if not the default template is used.
pub const TEMPLATE_FILE: &str = ".changelog.hbs";

/// The embedded template that is used when none is provided by the user.
const TEMPLATE_DEFAULT: &str = include_str!("assets/changelog.hbs");

/// The tool configuration.
///
/// The configuration defines the repository conventions and output preferences.
#[serde(default)]
#[derive(Debug, Default, Deserialize)]
pub struct Configuration {
    /// The project conventions
    pub conventions: Conventions,

    /// The output preferences
    pub output: OutputPreferences,
}

/// The change categorization conventions used by a repository/project.
#[serde(default)]
#[derive(Debug, Default, Deserialize)]
pub struct Conventions {
    /// The scope keywords
    pub scopes: Vec<Keyword>,

    /// The category keywords
    pub categories: Vec<Keyword>,
}

/// A keyword used to categorize commit message lines.
#[serde(default)]
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Keyword {
    /// The identifying tag used in commit messages.
    pub tag: String,

    /// The presentation title that shows up in the final change log.
    pub title: String,
}

/// The output preferences
#[serde(default)]
#[derive(Debug, Default, Deserialize)]
pub struct OutputPreferences {
    /// Output as JSON
    pub json: bool,

    /// Output Handlebar template
    pub template: Option<String>,

    /// The remote url
    pub remote: Option<String>,

    /// Output line post-processors
    pub post_processors: Vec<PostProcessor>,
}

/// A post-processor definition.
#[serde(default)]
#[derive(Debug, Default, Deserialize)]
pub struct PostProcessor {
    /// The lookup pattern
    pub lookup: String,

    /// The replace pattern
    pub replace: String,
}

impl Configuration {
    /// Construct from the given YAML string
    pub fn from_yaml(yml: &str) -> Result<Self> {
        from_str(yml).map_err(|e| format_err!("Configuration contains invalid YAML: {}", e))
    }

    /// Construct from the given YAML file
    pub fn from_file(file: Option<&str>) -> Result<Self> {
        file.map(str::to_owned)
            .or_else(|| find_file(CONFIG_FILE))
            .map_or_else(|| Ok(String::from(CONFIG_DEFAULT)), |f| read_file(&f))
            .and_then(|yml| Self::from_yaml(&yml))
    }
}

impl Conventions {
    /// Get the title for the given scope
    pub fn scope_title(&self, scope: Option<String>) -> Option<&str> {
        self.title(&self.scopes, scope)
    }

    /// Get the title for the given category
    pub fn category_title(&self, category: Option<String>) -> Option<&str> {
        self.title(&self.categories, category)
    }

    /// Get the titles for all the categories defined
    pub fn category_titles(&self) -> Vec<&str> {
        Self::titles(&self.categories)
    }

    /// Get the titles for all the scopes defined
    pub fn scope_titles(&self) -> Vec<&str> {
        Self::titles(&self.scopes)
    }

    /// Given the available keywords, get the title for the given tag
    fn title<'a>(&'a self, keywords: &'a [Keyword], tag: Option<String>) -> Option<&'a str> {
        // The least we have is a "blank" one.
        if keywords.is_empty() && tag.is_none() {
            return Some("");
        }

        // Look in the list for one that matches the given tag
        let given = tag.unwrap_or_default();
        for kw in keywords {
            if kw.tag == given {
                return Some(&kw.title);
            }
        }

        None
    }

    /// Given the available keywords, get a iterable list of the titles
    fn titles(keywords: &[Keyword]) -> Vec<&str> {
        if keywords.is_empty() {
            vec![""]
        } else {
            keywords.iter().map(|k| k.title.as_ref()).collect()
        }
    }
}

impl OutputPreferences {
    /// Get the template definition
    pub fn get_template(&self) -> Result<String> {
        self.template
            .clone()
            .or_else(|| find_file(TEMPLATE_FILE))
            .map_or_else(|| Ok(String::from(TEMPLATE_DEFAULT)), |f| read_file(&f))
    }
}

/// Read the given file to a String (with logging)
fn read_file(name: &str) -> Result<String> {
    // Return the data
    info!("Reading file '{}'", name);
    let mut contents = String::new();

    File::open(name)
        .map_err(|e| format_err!("Cannot open file '{}' (Reason: {})", name, e))?
        .read_to_string(&mut contents)
        .map_err(|e| format_err!("Cannot read file '{}' (Reason: {})", name, e))?;

    Ok(contents)
}

/// Identify the closest configuration file that should be used for this run
fn find_file(file: &str) -> Option<String> {
    // Start at the current directory
    let mut cwd = current_dir().expect("Current directory is invalid");

    // While we have hope
    while cwd.exists() {
        // Set the filename we're looking for
        cwd.push(file);

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

    // No file found
    None
}

#[cfg(test)]
mod tests {
    use super::Configuration;

    #[test]
    fn configuration_from_yaml() {
        let project = include_str!("../.changelog.yml");
        let no_category = r#"
        conventions:
          scopes: [{keyword:"a", title: "A"}]
        "#;
        let no_scope = r#"
        conventions:
          categories: [{keyword:"a", title: "A"}]
        "#;
        assert!(Configuration::from_yaml("").is_err());
        assert!(Configuration::from_yaml(project).is_ok());
        assert!(Configuration::from_yaml(no_scope).is_ok());
        assert!(Configuration::from_yaml(no_category).is_ok());
    }

    #[test]
    fn find_file() {
        use super::find_file;
        assert!(find_file("unknown").is_none());
        assert!(find_file("Cargo.toml").is_some());
    }
}
