pub mod args;
pub mod diagnostic;

use std::fs;
use crate::diagnostic::Diagnostic;


pub fn run(raw_args: Vec<String>) -> Result<(), Diagnostic> {
    let args = args::parse(raw_args)?;

    for path in args.paths {
        let contents = fs::read_to_string(&path)
            .map_err(|err| Diagnostic::failure(format!("surf: failed to read {path}: {err}")))?;

        let pattern = if args.ignore_case {
            &args.pattern.to_lowercase()
        } else {
            &args.pattern
        };

        for (index, line) in contents.lines().enumerate() {
            let haystack = if args.ignore_case {
                line.to_lowercase()
            } else {
                line.to_string()
            };

            if haystack.contains(pattern) {
                if args.line_numbers {
                    println!("{}: {line}", index + 1);
                } else {
                    println!("{line}");
                }
            }
        }
    }

    Ok(())
}
