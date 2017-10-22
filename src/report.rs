// Copyright 2017 Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

/// Presentation and Reporting
use chrono::Local;
use commit::Commit;
use config::Configuration;
use handlebars::Handlebars;
use std::collections::HashMap;
use serde_json::to_string_pretty;

/// The complete report
#[derive(Serialize)]
pub struct Report<'a> {
    /// The time of report generation
    pub date: String,
    /// The report title
    pub title: String,
    /// Scoped changes in the report
    pub scopes: Vec<Scope>,
    /// All interesting commits
    pub commits: &'a [Commit],
}

/// A group of changes in the same scope
#[derive(Serialize)]
pub struct Scope {
    /// The title of the scope
    pub title: String,
    /// A list of categorized changes
    pub categories: Vec<Category>,
}

/// A group of changes with the same category
#[derive(Serialize)]
pub struct Category {
    /// The title of the category
    pub title: String,
    /// A list of change descriptions
    pub changes: Vec<Text>,
}

/// Change description
#[derive(Serialize, Clone)]
pub struct Text {
    /// An opening headline
    pub opening: String,
    /// The remaining lines in the description
    pub rest: Vec<String>,
}

/// Generate a new report for the commits with the given configuration
pub fn generate<'a>(commits: &'a [Commit], config: &'a Configuration) -> Report<'a> {
    Report {
        commits,
        scopes: organize(config, commits),
        title: config.report.title.clone(),
        date: Local::now().format(&config.report.date_format).to_string(),
    }
}

/// Render the given report with the given configuration
pub fn render(report: &Report, config: &Configuration) -> String {
    Handlebars::new()
        .template_render(&config.report.template, report)
        .unwrap()
        .trim()
        .to_string()
}

/// A temporary report structure with look-ups on scope and category keys
type RawReport = HashMap<String, HashMap<String, Vec<Text>>>;

/// The running state kept during report construction
#[derive(Default, Clone, Serialize)]
struct State {
    /// The current text
    text: Vec<String>,
    /// The current scope
    scope: Option<String>,
    /// The current category
    category: Option<String>,
}

/// Record the current state into the raw report
fn record(raw: &mut RawReport, config: &Configuration, mut state: State) {
    // Validate the category with the configuration
    let category = config.categories.validate(state.category);

    // Validate the scope with the configuration
    let scope = config.scopes.validate(state.scope);

    // If the scope and category are known and we have some text to record
    if category.is_some() && scope.is_some() && !state.text.is_empty() {
        // Split the opening line and the remainder
        let opening = state.text.remove(0);
        let rest = state.text;

        // Record it in the raw report
        raw.entry(scope.unwrap())
            .or_insert_with(HashMap::new)
            .entry(category.unwrap())
            .or_insert_with(Vec::new)
            .push(Text { opening, rest });
    }
}

/// Group the commits into scopes
fn organize(config: &Configuration, commits: &[Commit]) -> Vec<Scope> {
    let report = &mut RawReport::new();

    // Take each commit
    for commit in commits {
        let mut current = State::default();

        // Take each line in the message
        for line in &commit.lines {
            // If this line opens a new category
            if line.category.is_some() {
                // Close out the current item
                record(report, config, current.clone());

                // Start a new context
                current.text = Vec::new();
                current.scope = line.scope.clone();
                current.category = line.category.clone();
            }

            // Record the line text
            current.text.push(line.text.clone().unwrap_or_default());
        }

        // Close the last open item
        record(report, config, current);
    }

    // Log the report for debugging
    info!("{}", to_string_pretty(&report).unwrap());

    // Finalize
    finish(report, config)
}

/// Create the final report from the given raw report and configuration
fn finish(report: &RawReport, config: &Configuration) -> Vec<Scope> {
    // The report of all scopes
    let mut scopes = Vec::new();

    // Go through each configured scope
    for scope in &config.scopes.tags {
        // If we have changes for the scope in the report
        if let Some(categorized) = report.get(scope) {
            // The scoped categorized changes
            let mut categories = Vec::new();

            // Go through all configured scopes
            for category in &config.categories.tags {
                // If there are changes of this category
                if let Some(changes) = categorized.get(category) {
                    // Add them to the running list
                    categories.push(Category {
                        title: config.categories.get_title(category),
                        changes: changes.clone(),
                    });
                }
            }

            // Record them in the scopes list
            scopes.push(Scope {
                title: config.scopes.get_title(scope),
                categories,
            });
        }
    }

    // Done
    scopes
}
