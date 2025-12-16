use std::{env, process};

use crate::config::Config;

mod matcher;
mod config;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let cfg = Config::new(&args)
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1)
        }
    );
        
    let found = matcher::read_lines(&cfg.filename).unwrap();
    for (idx, line) in matcher::find_matches(&cfg.pattern, &found) {
        println!("{}: {}", idx, line)
    }
}
