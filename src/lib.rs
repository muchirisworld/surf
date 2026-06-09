use std::fs;

struct Args {
    pattern: String,
    path: String,
}

pub fn run(args: Vec<String>) -> Result<(), String> {
    let args = parse_args(args)?;

    let contents = fs::read_to_string(&args.path)
        .map_err(|err| format!("surf: failed to read {}: {err}", args.path))?;

    for line in find_matches(&args.pattern, contents.as_str()) {
        println!("{line}");
    }

    Ok(())
}

fn parse_args(args: Vec<String>) -> Result<Args, String> {
    if args.len() != 2 {
        return Err("usage: surf <pattern> <path>".to_string());
    }

    Ok(Args {
        pattern: args[0].clone(),
        path: args[1].clone(),
    })
}

fn find_matches<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|x| x.contains(pattern)).collect()
}
