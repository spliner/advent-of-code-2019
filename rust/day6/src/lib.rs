use std::error::Error;


pub mod config;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    match config.part {
        config::Part::PartOne => {
            println!("Part one will go here");
        }
        config::Part::PartTwo => {
            println!("Part two will go here");
        }
    }

    Ok(())
}
