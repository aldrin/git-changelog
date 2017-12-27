// Copyright 2017 Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

// Output concerns
use handlebars::Handlebars;
use regex::Regex;
use super::{ChangeLog, PostProcessor};
use std::io::{Error, ErrorKind};

pub fn from_hbs(given: Option<String>) -> Result<Handlebars, Error> {
    let mut hbs = Handlebars::new();
    let md = given.unwrap_or_else(|| String::from(include_str!("assets/changelog.hbs")));
    hbs.register_template_string("default", md).map_err(|e| {
        error!("Invalid handlebar template: '{}'", e);
        Error::from(ErrorKind::InvalidInput)
    })?;
    Ok(hbs)
}

pub fn render(log: &ChangeLog, hbs: &Handlebars) -> String {
    hbs.render("default", log).unwrap().trim().to_string()
}

/// Postprocess the output before printing it out
pub fn postprocess(post_processors: &[PostProcessor], output: &str) -> String {
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

    #[test]
    fn from_hbs() {
        use super::from_hbs;
        assert!(from_hbs(None).is_ok());
        assert!(from_hbs(Some(String::from("{{bad"))).is_err());
    }

    #[test]
    fn postprocess() {
        let input = String::from("Fixed JIRA-1234\nfoo");
        let mut jira = super::PostProcessor::default();
        jira.lookup = r"JIRA-(?P<t>\d+)".to_string();
        jira.replace = r"[JIRA-$t](https://our.jira/$t)".to_string();
        let out = super::postprocess(&vec![jira], &input);
        assert_eq!(&out, "Fixed [JIRA-1234](https://our.jira/1234)\nfoo");

        let mut bad = super::PostProcessor::default();
        bad.lookup = r"JIRA-?(P<t\d+".to_string();
        bad.replace = r"whatever".to_string();
        let out = super::postprocess(&vec![bad], &input);
        assert_eq!(&out, &input);
    }
}
