// Copyright 2017 Aldrin J D'Souza.
// Licensed under the MIT License <https://opensource.org/licenses/MIT>

/// Commit parser
use std::str;
use chrono::{DateTime, Local};
use nom::{is_alphanumeric, IResult};

/// A single commit
#[derive(Serialize, Debug)]
pub struct Commit {
    /// The change SHA
    pub sha: String,
    /// The change author
    pub author: String,
    /// The change timestamp
    pub time: String,
    /// The change summary
    pub summary: String,
    /// The change number
    pub number: Option<u32>,
    /// The message lines
    pub lines: Vec<Line>,
}

/// A single line in the change message
#[derive(Default, Serialize, Debug)]
pub struct Line {
    /// The scope
    pub scope: Option<String>,
    /// The category
    pub category: Option<String>,
    /// The text
    pub text: Option<String>,
}

/// Parse the commit message to a Commit object
pub fn parse(lines: &[String], dt_format: &str) -> Commit {
    Commit {
        sha: lines[0].clone(),
        author: lines[1].clone(),
        number: parse_number(&lines[3]),
        summary: parse_subject(&lines[3]),
        time: parse_time(&lines[2], dt_format),
        lines: lines[4..].iter().map(|s| parse_line(s)).collect(),
    }
}

/// Parse the commit timestamp to local time
pub fn parse_time(line: &str, format: &str) -> String {
    DateTime::parse_from_rfc2822(line)
        .map(|t| t.with_timezone(&Local))
        .unwrap_or_else(|_| Local::now())
        .format(format)
        .to_string()
}

/// Parse the commit subject removing the numbers tags.
pub fn parse_subject(line: &str) -> String {
    let first_open = line.find("(#").unwrap_or_else(|| line.len());
    String::from(line.get(0..first_open).unwrap_or_else(|| line).trim())
}

/// Parse the commit number
pub fn parse_number(line: &str) -> Option<u32> {
    let last_open = line.rfind("(#");
    let last_close = line.rfind(')');
    if last_open.is_none() || last_close.is_none() {
        None
    } else {
        let end = last_close.unwrap();
        let start = last_open.unwrap() + "(#".len();
        let num = line.get(start..end).map(|s| s.parse().ok());
        num.unwrap_or(None)
    }
}

/// Parse an individual message line
pub fn parse_line(line: &str) -> Line {
    match tagged_change(line) {
        IResult::Done(_, l) => l,
        _ => Line::default(),
    }
}

/// A change message line is one of the following types
named!(tagged_change<&str, Line>,
       alt!(
           category_scope_change
               | category_scope
               | category_change
               | just_category
               | just_change
       ));

/// A line that has everything, i.e. category, scope and a change.
named!(category_scope_change<&str, Line>,
       do_parse!(
           tag!("-") >> category: tagname >>
               tag!("(") >> scope: tagname >>
               tag!("):") >> text: whatever >>
               (Line{
                   scope: Some(scope),
                   category: Some(category),
                   text: Some(text),
                })));

/// A line that has just a category and scope, but no text
named!(category_scope<&str, Line>,
       do_parse!(
           tag!("-") >> category: tagname >>
               tag!("(") >> scope: tagname >>
               tag!("):") >>
               (Line{
                   scope: Some(scope),
                   category: Some(category),
                   text: None,
               })));

/// A line that has just a category
named!(just_category<&str, Line>,
       do_parse!(
           tag!("-") >> category: tagname >>
               tag!(":") >>
               (Line{
                   scope: None,
                   category: Some(category),
                   text: None,
               })));

/// A line that has a category and a change, but no scope.
named!(category_change<&str, Line>,
       do_parse!(
           tag!("-") >> category: tagname >>
               tag!(":") >> text: whatever >>
               (Line{
                   scope: None,
                   category: Some(category),
                   text: Some(text),
               })));

/// A line that has just a simple change (no tags).
named!(just_change<&str, Line>,
       do_parse!(opt!(tag!("-")) >>
                 text: whatever >>
                 (Line{
                     scope: None,
                     category: None,
                     text: Some(text),
                 })));

/// Consume whatever is left and return a String
named!(whatever<&str, String>,
       map!(take_while1_s!(|_| true), String::from));

/// Consume an acceptable tag name and return a String
named!(tagname<&str, String>,
       map!(ws!(take_while1_s!(|c| is_alphanumeric(c as u8))), str::to_lowercase));
