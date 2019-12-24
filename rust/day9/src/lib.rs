use std::error::Error;
use std::fs;

use intcode::Intcode;

pub mod config;
pub mod intcode;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let program: Vec<i64> = fs::read_to_string(config.filename)?
        .split(",")
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect();

    match config.part {
        config::Part::PartOne => {
            let mut intcode = Intcode::new(&program);
            intcode.compute()?;
            println!("{:?}", intcode.outputs());
        }
        config::Part::PartTwo => {
            // TODO: Part two
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {}
