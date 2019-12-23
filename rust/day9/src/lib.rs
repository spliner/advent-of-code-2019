use std::error::Error;
use std::fs;

pub mod config;
pub mod intcode;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let _program: Vec<i32> = fs::read_to_string(config.filename)?
        .split(",")
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();

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
mod tests {}
