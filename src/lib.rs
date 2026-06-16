use std::fs;

pub mod args;

pub fn run(raw_args: Vec<String>) -> Result<(), String> {
    let args = args::parse(raw_args).map_err(|err| format!("{err:?}"))?;

    for path in args.paths {
        let contents = fs::read_to_string(&path)
            .map_err(|err| format!("surf: failed to read {path}: {err}"))?;

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
