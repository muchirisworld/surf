use std::path::PathBuf;

use crate::{
    config,
    diagnostic::{Diagnostic, ExitCode},
};

pub struct Cli {
    pub pattern: String,
    pub paths: Vec<String>,
    pub options: config::FileConfig,
}

pub fn parse<I>(args: I) -> Result<Cli, Diagnostic>
where
    I: IntoIterator<Item = String>,
{
    let mut tokens = args.into_iter().peekable();
    let mut recursive = None;
    let mut line_numbers = None;
    let mut ignore_case = None;
    let mut invert_match = None;
    let mut whole_line = None;
    let mut before_context = None;
    let mut after_context = None;
    let mut color = None;
    let mut ignore_file = None;

    let mut positionals = Vec::new();

    while let Some(token) = tokens.next() {
        match token.as_str() {
            "-r" | "--recursive" => recursive = Some(true),
            "-n" | "--line-numbers" => line_numbers = Some(true),
            "-i" | "--ignore-case" => ignore_case = Some(true),
            "-x" | "--line-regexp" => whole_line = Some(true),
            "-v" | "--invert-match" => invert_match = Some(true),

            // Parsing arguments that take a value:
            "-B" | "--before-context" => {
                let val_str = tokens.next().ok_or_else(|| {
                    Diagnostic::usage("Missing value for --before-context".to_string())
                })?;
                let val = val_str.parse::<usize>().map_err(|_| {
                    Diagnostic::usage(format!(
                        "Invalid integer value `{val_str}` for --before-context"
                    ))
                })?;
                before_context = Some(val);
            }
            "-A" | "--after-context" => {
                let val_str = tokens.next().ok_or_else(|| {
                    Diagnostic::usage("Missing value for --after-context".to_string())
                })?;
                let val = val_str.parse::<usize>().map_err(|_| {
                    Diagnostic::usage(format!(
                        "Invalid integer value `{val_str}` for --after-context"
                    ))
                })?;
                after_context = Some(val);
            }
            "--color" => {
                let color_str = tokens
                    .next()
                    .ok_or_else(|| Diagnostic::usage("Missing value for --color".to_string()))?;
                color = Some(color_str);
            }
            "--ignore-file" => {
                let path_str = tokens.next().ok_or_else(|| {
                    Diagnostic::usage("Missing value for --ignore-file".to_string())
                })?;
                ignore_file = Some(PathBuf::from(path_str));
            }
            "-h" | "--help" => {
                return Err(Diagnostic {
                    code: ExitCode::Success,
                    message: "surf searches files for matching lines".to_string(),
                    help: Some("usage: rgrep [OPTIONS] <pattern> <path>...".to_string()),
                });
            }
            "--" => {
                positionals.extend(tokens);
                break;
            }
            flag if flag.starts_with('-') => {
                return Err(Diagnostic::usage(format!("Unknown flag `{token}`")));
            }
            _ => positionals.push(token),
        }
    }

    let mut positionals = positionals.into_iter();
    let pattern = positionals
        .next()
        .ok_or_else(|| Diagnostic::usage("Missing pattern"))?;
    let paths: Vec<String> = positionals.collect();

    if paths.is_empty() {
        return Err(Diagnostic::usage("Missing path"));
    }

    Ok(Cli {
        pattern,
        paths,
        options: config::FileConfig {
            recursive,
            line_numbers,
            ignore_case,
            invert_match,
            whole_line,
            after_context,
            before_context,
            color,
            ignore_file,
        },
    })
}
