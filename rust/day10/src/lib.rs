use std::error::Error;
use std::fs;

pub mod config;
pub mod point;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let _input = fs::read_to_string(config.filename)?;

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

#[cfg(test)]
mod tests {
    use super::*;
}
