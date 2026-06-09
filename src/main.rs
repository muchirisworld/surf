use std::{env, fs, process};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("usage: rgrep <pattern> <path>");
        process::exit(2)
    }

    let pattern = &args[1];
    let target = &args[2];

    let contents = match fs::read_to_string(target) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("surf: failed to read {target}: {err}");
            process::exit(1);
        }
    };

    for line in find_matches(pattern, contents.as_str()) {
        println!("{line}");
    }
    
}

fn find_matches<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|x| {
            x.contains(pattern)
        })
        .collect()
}