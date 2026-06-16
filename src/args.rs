
pub struct Cli {
    pub pattern: String,
    pub paths: Vec<String>,
    pub recursive: bool,
    pub line_numbers: bool,
    pub ignore_case: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    MissingPattern,
    MissingPath,
    UnknownFlag(String),
    MissingFlagValue(&'static str),
}

pub fn parse<I>(args: I) -> Result<Cli, ParseError>
where
    I: IntoIterator<Item = String>
{
    let mut tokens = args.into_iter().peekable();
    let mut recursive = false;
    let mut line_numbers = false;
    let mut ignore_case = false;
    let mut positionals = Vec::new();

    while let Some(token) = tokens.next() {
        match token.as_str() {
            "-r" | "--recursive" => recursive = true,
            "-n" | "--line-numbers" => line_numbers = true,
            "-i" | "--ignore-case" => ignore_case = true,
            "--" => {
                positionals.extend(tokens);
                break;
            }
            flag if flag.starts_with('-') => return Err(ParseError::UnknownFlag(token)),
            _ => positionals.push(token),
        }
    }

    let mut positionals = positionals.into_iter();
    let pattern = positionals.next().ok_or(ParseError::MissingPattern)?;
    let paths: Vec<String> = positionals.collect();

    if paths.is_empty() {
        return Err(ParseError::MissingPath)
    }

    Ok(Cli{
        pattern,
        paths,
        recursive,
        line_numbers,
        ignore_case,
    })
    
}