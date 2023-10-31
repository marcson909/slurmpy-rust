use std::{time::Duration, fmt::Error};
use chrono::{NaiveDate, ParseError};
use clap::Args;

/// Build an extremely limited grep command with an optional fallback
#[derive(Args, Debug)]
pub struct GreprArgs {
    /// The pattern to look for
    pub pattern: String,
    /// The path to the file to read
    pub path: std::path::PathBuf,
    /// The text to show if no pattern was matched
    pub fallback_text: Vec<String>,
}