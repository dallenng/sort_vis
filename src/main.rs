use std::{env, process};

fn main() {
    let config = sort_vis::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(err) = sort_vis::run(config) {
        eprintln!("{}", err);
        process::exit(2);
    }
}
