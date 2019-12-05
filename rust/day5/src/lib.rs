use std::error::Error;
use std::fs;

pub mod config;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let _contents = fs::read_to_string(config.filename)?;

    match config.part {
        config::Part::PartOne => {
            // TODO: Part one
        }
        config::Part::PartTwo => {
            // TODO: Part two
        }
    }

    Ok(())
}
