pub mod args;
pub mod diagnostic;
pub mod search;

use crate::diagnostic::Diagnostic;
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
        let options = search::SearchOptions {
            ignore_case: args.ignore_case,
            pattern: args.pattern.clone(),
            recursive: args.recursive,
        };

        let matches = search::search_reader(reader, &options)
            .map_err(|err| Diagnostic::failure(format!("failed to read {path}: {err}")))?;
        search::write_matches(&mut out, &matches, args.line_numbers)
            .map_err(|err| Diagnostic::failure(format!("failed to write output: {err}")))?;
    }

    Ok(())
}
