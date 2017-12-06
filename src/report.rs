// Copyright 2017 Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

/// Presentation and Reporting
use config;
use commit::Commit;
use config::Configuration;
use std::collections::HashMap;
use serde_json::to_string_pretty;

/// The complete report
#[derive(Serialize)]
pub struct Report<'a> {
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

/// Generate a new report for the commits with the given configuration
pub fn generate<'a>(config: &'a Configuration, commits: &'a [Commit]) -> Report<'a> {

    // First pass - categorize
    let raw_report = first_pass(config, commits);

    // Second pass - aggregate
    let scopes = second_pass(config, &raw_report);

    // Done
    Report { commits, scopes }
}

/// The first pass - walks through commits and gathers scopes and categories.
fn first_pass(config: &Configuration, commits: &[Commit]) -> RawReport {

    // A running raw report
    let mut raw_report = RawReport::new();

    // Take each commit
    for commit in commits {

        // Initialize a fresh current
        let mut current = State::default();

        // Take each line in the message
        for line in &commit.lines {

            // If this line opens a new category
            if line.category.is_some() {

                // Close out the current item
                record(&mut raw_report, config, current.clone());

                // Start a new context
                current.text = Vec::new();
                current.scope = line.scope.clone();
                current.category = line.category.clone();
            }

            // Record the line text
            current.text.push(line.text.clone().unwrap_or_default());
        }

        // Close the last open item
        record(&mut raw_report, config, current);
    }

    // Log the raw_report for debugging
    info!("RAW_REPORT: {}", to_string_pretty(&raw_report).unwrap());

    raw_report
}

/// The second pass takes the raw report and orders things as we want to show them
fn second_pass(config: &Configuration, report: &RawReport) -> Vec<Scope> {

    // The report of all scopes
    let mut scopes = Vec::new();

    // Go through each configured scope
    for scope in &config.scopes {

        // If we have changes for the scope in the report
        if let Some(categorized) = report.get(&scope.title) {

            // The scoped categorized changes
            let mut categories = Vec::new();

            // Go through all configured scopes
            for category in &config.categories {

                // If there are changes of this category
                if let Some(changes) = categorized.get(&category.title) {

                    // Add them to the running list
                    categories.push(Category {
                        title: category.title.clone(),
                        changes: changes.clone(),
                    });
                }
            }

            // Record them in the scopes list
            scopes.push(Scope {
                title: scope.title.clone(),
                categories,
            });
        }
    }

    // Done
    scopes
}

/// Record the current state into the raw report
fn record(raw: &mut RawReport, config: &Configuration, mut state: State) {

    // Validate the scope with the configuration
    let scope = config::report_title(&config.scopes, &state.scope);

    // Validate the category with the configuration
    let category = config::report_title(&config.categories, &state.category);

    // If the scope and category are known and we have some text to record
    if category.is_some() && scope.is_some() && !state.text.is_empty() {

        // Split the opening line and the remainder
        let mut opening = state.text.remove(0);

        // If the opening line is empty,
        while opening.trim().is_empty() {

            // take the next one
            opening = state.text.remove(0)
        }

        // If the opening line has no buffer space,
        if !opening.starts_with(' ') {

            // add it
            opening.insert(0, ' ');
        }

        // Take the rest
        let rest = state.text;

        // Record it in the raw report
        raw.entry(scope.unwrap())
            .or_insert_with(HashMap::new)
            .entry(category.unwrap())
            .or_insert_with(Vec::new)
            .push(Text { opening, rest });
    }
}
