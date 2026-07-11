use std::io::{self, BufRead, Write};

use crate::matcher::Matcher;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SearchOptions {
    pub pattern: String,
    pub recursive: bool,
    pub ignore_case: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LineMatch {
    pub line: String,
    pub line_number: usize,
}

pub fn search_reader<R>(reader: R, matcher: &Matcher) -> io::Result<Vec<LineMatch>>
where
    R: BufRead,
{
    let mut matches = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;

        if matcher.is_match(&line) {
            matches.push(LineMatch {
                line,
                line_number: i + 1,
            });
        }
    }

    Ok(matches)
}

pub fn write_matches<W>(mut writer: W, matches: &[LineMatch], line_numbers: bool) -> io::Result<()>
where
    W: Write,
{
    for item in matches {
        if line_numbers {
            writeln!(writer, "{}:{}", item.line_number, item.line)?
        } else {
            writeln!(writer, "{}", item.line)?
        }
    }
    Ok(())
}
