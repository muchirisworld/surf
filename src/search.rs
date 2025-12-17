use std::{io::Error};

use crate::matcher::{Matcher, read_lines};

#[derive(Debug)]
pub struct Match {
    pub line_number: usize,
    pub line: String,
    pub start: usize,
    pub end: usize
}

pub fn search_file(
    matcher: &dyn Matcher,
    path: &str
) -> Result<Vec<Match>, Error> {
    let lines_read = read_lines(path)?;
        
    Ok(
        lines_read
            .into_iter()
            .enumerate()
            .filter_map(|(idx, line)| {
                matcher.find(line.as_str())
                    .map(|(start, end)| Match {
                        line_number: idx+1,
                        line,
                        start,
                        end
                    })
            })
            .collect::<Vec<_>>()
    )
}
