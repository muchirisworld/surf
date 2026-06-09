use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match surf::run(args) {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    }
}
