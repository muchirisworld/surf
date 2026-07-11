pub mod args;
pub mod diagnostic;
pub mod matcher;
pub mod search;
pub mod walk;

use crate::{
    diagnostic::Diagnostic,
    matcher::{MatchMode, Matcher},
};
use std::{
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
};

pub fn run(raw_args: Vec<String>) -> Result<(), Diagnostic> {
    let args = args::parse(raw_args)?;
    let mut out = io::stdout().lock();

    let paths: Vec<PathBuf> = args.paths.iter().map(PathBuf::from).collect();
    let files = walk::collect_files(
        &paths,
        &walk::WalkOptions {
            recursive: args.recursive,
        },
    )
    .map_err(|err| Diagnostic::failure(format!("failed to collect files: {err}")))?;

    for item in files {
        let file = File::open(&item.path)
            .map_err(|err| Diagnostic::failure(format!("surf: failed to open {:?}: {err}", item.path)))?;
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
            args.invert_match,
        );

        let matches = search::search_reader(reader, &matcher)
            .map_err(|err| Diagnostic::failure(format!("failed to read {:?}: {err}", item.path)))?;
        search::write_matches(&mut out, &matches, args.line_numbers)
            .map_err(|err| Diagnostic::failure(format!("failed to write output: {err}")))?;
    }

    Ok(())
}
