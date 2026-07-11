pub mod args;
pub mod diagnostic;
pub mod matcher;
pub mod search;

use crate::{diagnostic::Diagnostic, matcher::{MatchMode, Matcher}};
use std::{
    fs::File,
    io::{self, BufReader},
};

pub fn run(raw_args: Vec<String>) -> Result<(), Diagnostic> {
    let args = args::parse(raw_args)?;
    let mut out = io::stdout().lock();

    for path in args.paths {
        let file = File::open(&path)
            .map_err(|err| Diagnostic::failure(format!("surf: failed to open {path}: {err}")))?;
        let reader = BufReader::new(file);
        
        let mode = if args.whole_line {
            MatchMode::WholeLine
        } else {
            MatchMode::Contains
        };
        let matcher = Matcher::new(
            args.pattern.clone(),
            args.ignore_case,
            mode,
            args.invert_match
        );

        let matches = search::search_reader(reader, &matcher)
            .map_err(|err| Diagnostic::failure(format!("failed to read {path}: {err}")))?;
        search::write_matches(&mut out, &matches, args.line_numbers)
            .map_err(|err| Diagnostic::failure(format!("failed to write output: {err}")))?;
    }

    Ok(())
}
