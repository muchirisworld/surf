use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match surf::run(args) {
        Ok(()) => {}
        Err(diag) => {
            eprintln!("{diag}");
            process::exit(diag.code as i32);
        }
    }
}
