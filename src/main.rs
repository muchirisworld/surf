use std::{env, process};

use surf::{Config, LiteralMatcher, Matcher, search_file};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let cfg = Config::new(&args)
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1)
        }
    );
    
    let matcher: Box<dyn Matcher> = Box::new(
        LiteralMatcher::new(&cfg.pattern)
    );
    
    match search_file(matcher.as_ref(), &cfg.filename) {
        Ok(matches) => {
            println!("{:?}", matches)
        },
        Err(e) => eprintln!("Error occurred! {e}")
    }
    
}
 