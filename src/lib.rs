pub mod args;
pub mod config;
pub mod diagnostic;
pub mod ignore;
pub mod matcher;
pub mod parallel;
pub mod render;
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
            ignore: &ignore::IgnoreSet::empty(),
        },
    )
    .map_err(|err| Diagnostic::failure(format!("failed to collect files: {err}")))?;

    for item in files {
        let file = File::open(&item.path).map_err(|err| {
            Diagnostic::failure(format!("surf: failed to open {:?}: {err}", item.path))
        })?;
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

        let context = search::Context {
            before: 0,
            after: 0,
        };
        let events = search::search_reader(reader, &matcher, context)
            .map_err(|err| Diagnostic::failure(format!("failed to read {:?}: {err}", item.path)))?;
        let options = render::RenderOptions {
            line_numbers: args.line_numbers,
            color: render::Color::Never,
        };
        render::render_events(&mut out, &item.path, &events, options)
            .map_err(|err| Diagnostic::failure(format!("failed to write output: {err}")))?;
    }

    Ok(())
}
