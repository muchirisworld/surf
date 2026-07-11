use std::io::{self, BufRead, Write};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SearchOptions {
    pub pattern: String,
    pub recursive: bool,
    pub ignore_case: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Match {
    pub line: String,
    pub line_number: usize,
}

pub fn search_reader<R>(reader: R, options: &SearchOptions) -> io::Result<Vec<Match>>
where
    R: BufRead,
{
    let needle = if options.ignore_case {
        options.pattern.to_lowercase()
    } else {
        options.pattern.clone()
    };

    let mut matches = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;

        let haystack = if options.ignore_case {
            line.to_lowercase()
        } else {
            line.clone()
        };

        if haystack.contains(&needle) {
            matches.push(Match {
                line,
                line_number: i + 1,
            });
        }
    }

    Ok(matches)
}

pub fn write_matches<W>(mut writer: W, matches: &[Match], line_numbers: bool) -> io::Result<()>
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
