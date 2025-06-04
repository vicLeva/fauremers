use std::{env, process};

use fauremers::Config;

fn main() {
    let expes: bool = true;

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if expes {
        if let Err(e) = fauremers::run_expes(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
        } 
    } else {
        if let Err(e) = fauremers::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
        }
    }

}
