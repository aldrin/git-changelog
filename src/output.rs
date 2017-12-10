// Copyright 2017 Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

// Output concerns

use regex::Regex;
use report::Report;
use config::Configuration;
use handlebars::Handlebars;

/// Render the given report with the given configuration
pub fn render(config: &Configuration, report: &Report) {

    // Render the report using Handlebars
    let mut out = Handlebars::new()
        .template_render(&config.template, report)
        .unwrap()
        .trim()
        .to_string();

    // If we have line post processors,
    if !config.post_processors.is_empty() {
        // Post-process the output lines
        out = postprocess(&config, out);
    }

    // Print it out
    println!("{}", out);
}

/// Postprocess the output before printing it out
pub fn postprocess(config: &Configuration, output: String) -> String {

    // Compile the post processor regular expressions
    let mut processors = Vec::new();
    for processor in &config.post_processors {

        // Processor the lookup regular expression
        if let Ok(lookup) = Regex::new(&processor.lookup) {

            // Inform
            info!("Using post-processor {:#?}", lookup);

            // Remember the regex and the replacement string
            processors.push((lookup, processor.replace.as_str()));
        } else {

            // Invalid regex, warn and ignore
            warn!("Post-processor {:#?} is invalid", processor);
        }
    }

    // Track the processed output
    let mut processed = Vec::new();

    // Take each line in the output
    for line in output.lines() {

        // Make a mutable copy
        let mut next: String = line.to_string();

        // Run all available processors through it
        for processor in &processors {

            // Replace the pattern as appropriate
            next = processor.0.replace_all(&next, processor.1).to_string();
        }

        // Replace line with the processed
        processed.push(next);
    }

    // Return what we ended with
    processed.join("\n")
}
