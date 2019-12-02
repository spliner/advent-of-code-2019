
use std::error::Error;
use std::fs;

pub mod config;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let _contents = fs::read_to_string(config.filename)?;

    match config.part {
        config::Part::Part1 => {
            // TODO: Part 1
        }
        config::Part::Part2 => {
            // TODO: Part 2
        }
    }

    Ok(())
}

