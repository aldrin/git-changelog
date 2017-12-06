// Copyright 2017 Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

// Output concerns

use report::Report;
use config::Configuration;
use handlebars::Handlebars;

/// Render the given report with the given configuration
pub fn render(config: &Configuration, report: &Report) {
    let out = Handlebars::new()
        .template_render(&config.template, report)
        .unwrap()
        .trim()
        .to_string();

    println!("{}", out);
}
