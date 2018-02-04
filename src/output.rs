// Copyright 2017-2018 by Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

/// All output concerns.
use std::fmt;
use regex::Regex;
use handlebars::{Handlebars, Helper, RenderContext, RenderError};
use serde_json::to_string_pretty;
use super::Result;
use changelog::ChangeLog;
use input::{OutputPreferences, PostProcessor};

type RenderResult = ::std::result::Result<(), RenderError>;

/// Render the changelog with the given output preferences
pub fn render(clog: &ChangeLog, out: &OutputPreferences) -> Result<String> {
    // Depending on the output format, render the log to text
    let text = if out.json {
        to_string_pretty(clog).map_err(|e| format_err!("JSON render failed: {}", e))
    } else {
        let mut hbs = Handlebars::new();
        hbs.register_helper("tidy-change", Box::new(tidy));
        hbs.template_render(&out.get_template()?, clog)
            .map_err(|e| format_err!("Handlebar render failed: {}", e))
    };

    // Run the post processors on the output
    text.map(|s| post_process(&s, &out.post_processors))
}

/// A handlebar helper to tidy up markdown lists used to render changes.
fn tidy(h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> RenderResult {
    if let Some(indent) = h.param(0).and_then(|v| v.value().as_str()) {
        if let Some(text) = h.param(1).and_then(|v| v.value().as_str()) {
            let mut lines = text.lines();
            if let Some(first) = lines.next() {
                writeln!(rc.writer, "{}", first.trim())?;
            }
            for line in lines {
                writeln!(rc.writer, "{}{}", indent, line)?;
            }
        }
    }
    Ok(())
}

impl fmt::Display for ChangeLog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = OutputPreferences::default();
        match render(self, &out) {
            Ok(fine) => write!(f, "{}", fine),
            Err(err) => write!(f, "Error: {}", err),
        }
    }
}

/// Post process the output before returning it
fn post_process(output: &str, post_processors: &[PostProcessor]) -> String {
    // Compile the post processor regular expressions
    let mut processors = Vec::new();
    for processor in post_processors {
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

#[cfg(test)]
mod tests {
    use super::PostProcessor;

    #[test]
    fn post_process() {
        let input = String::from("Fixed JIRA-1234\nfoo");
        let mut jira = PostProcessor::default();
        jira.lookup = r"JIRA-(?P<t>\d+)".to_string();
        jira.replace = r"[JIRA-$t](https://our.jira/$t)".to_string();
        let out = super::post_process(&input, &vec![jira]);
        assert_eq!(&out, "Fixed [JIRA-1234](https://our.jira/1234)\nfoo");

        let mut bad = PostProcessor::default();
        bad.lookup = r"JIRA-?(P<t\d+".to_string();
        bad.replace = r"whatever".to_string();
        let out = super::post_process(&input, &vec![bad]);
        assert_eq!(&out, &input);
    }
}
