pub mod args;
pub mod config;
pub mod diagnostic;
pub mod ignore;
pub mod matcher;
pub mod parallel;
pub mod render;
pub mod search;
pub mod walk;

use crate::{diagnostic::Diagnostic, matcher::Matcher};
use std::{
    io::{self},
    path::PathBuf,
    thread,
};

pub fn run(raw_args: Vec<String>) -> Result<(), Diagnostic> {
    let args = args::parse(raw_args)?;
    let settings = config::Settings::build(&args.options)
        .map_err(|err| Diagnostic::failure(format!("failed to load settings: {err}")))?;
    let mut out = io::stdout().lock();

    let paths: Vec<PathBuf> = args.paths.iter().map(PathBuf::from).collect();
    let ignore_set = if let Some(ref path) = settings.ignore_file {
        ignore::IgnoreSet::from_file(path).map_err(|err| {
            Diagnostic::failure(format!("failed to load ignore file {:?}: {err}", path))
        })?
    } else {
        ignore::IgnoreSet::empty()
    };

    let files = walk::collect_files(
        &paths,
        &walk::WalkOptions {
            recursive: settings.recursive,
            ignore: &ignore_set,
        },
    )
    .map_err(|err| Diagnostic::failure(format!("failed to collect files: {err}")))?;

    let files: Vec<PathBuf> = files.into_iter().map(|item| item.path).collect();

    let matcher = Matcher::new(
        args.pattern.clone(),
        settings.ignore_case,
        settings.mode,
        settings.invert_match,
    );
    let context = search::Context {
        before: settings.before_context,
        after: settings.after_context,
    };
    let workers = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);

    let search_results = parallel::search_parallel(files, matcher, context, workers);

    let render_options = render::RenderOptions {
        line_numbers: settings.line_numbers,
        color: settings.color,
    };

    for result in search_results {
        match result.result {
            Ok(events) => {
                render::render_events(&mut out, &result.path, &events, render_options)
                    .map_err(|err| Diagnostic::failure(format!("failed to write output: {err}")))?;
            }
            Err(err_msg) => {
                eprintln!("surf: error searching {:?}: {}", result.path, err_msg);
            }
        }
    }

    Ok(())
}
