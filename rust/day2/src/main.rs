use std::{env, process};

use day2::config;

fn main() {
    let config = config::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);

        process::exit(1);
    });

    if let Err(e) = day2::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
