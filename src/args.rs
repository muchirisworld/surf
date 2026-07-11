use crate::diagnostic::{Diagnostic, ExitCode};

pub struct Cli {
    pub pattern: String,
    pub paths: Vec<String>,
    pub recursive: bool,
    pub line_numbers: bool,
    pub ignore_case: bool,
    pub invert_match: bool,
    pub whole_line: bool,
}

pub fn parse<I>(args: I) -> Result<Cli, Diagnostic>
where
    I: IntoIterator<Item = String>,
{
    let mut tokens = args.into_iter().peekable();
    let mut recursive = false;
    let mut line_numbers = false;
    let mut ignore_case = false;
    let mut invert_match = false;
    let mut whole_line = false;
    let mut positionals = Vec::new();

    while let Some(token) = tokens.next() {
        match token.as_str() {
            "-r" | "--recursive" => recursive = true,
            "-n" | "--line-numbers" => line_numbers = true,
            "-i" | "--ignore-case" => ignore_case = true,
            "-x" | "--line-regexp" => whole_line = true,
            "-v" | "--invert-match" => invert_match = true,
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
        recursive,
        line_numbers,
        ignore_case,
        invert_match,
        whole_line,
    })
}
