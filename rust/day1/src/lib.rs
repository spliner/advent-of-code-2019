use std::cmp::max;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::num::ParseIntError;

pub enum Part {
    Part1,
    Part2,
}

impl Part {
    pub fn new(raw_value: String) -> Result<Self, String> {
        match raw_value.to_lowercase().as_str() {
            "part1" => Ok(Part::Part1),
            "part2" => Ok(Part::Part2),
            _ => Err(format!("Invalid part: {}", raw_value)),
        }
    }
}

pub struct Config {
    pub filename: String,
    pub part: Part,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, String> {
        args.next();

        let part = match args.next() {
            Some(raw_part) => {
                match Part::new(raw_part) {
                    Ok(p) => p,
                    Err(e) => return Err(e),
                }
            },
            None => return Err(String::from("Didn't get a part")),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("Didn't get a file name")),
        };

        Ok(Config { filename, part, })
    }
}

struct Module {
    mass: i32,
}

impl Module {
    pub fn new(mass: i32) -> Self {
        Self { mass }
    }

    pub fn required_fuel(&self) -> i32 {
        Self::calculate_fuel(self.mass)
    }

    fn calculate_fuel(mass: i32) -> i32 {
        max(mass / 3 - 2, 0)
    }

    pub fn recursive_fuel(&self) -> i32 {
        let mut mass = *&self.mass;
        let mut required_fuel = 0;

        while mass > 0 {
            let fuel = Self::calculate_fuel(mass);

            required_fuel += fuel;
            mass = fuel;
        }

        required_fuel
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let modules = parse_modules(&contents);

    match config.part {
        Part::Part1 => {
            let result = part1(&modules);
            println!("{}", result);
        },
        Part::Part2 => {
            let result = part2(&modules);
            println!("{}", result);
        }
    }

    Ok(())
}

fn parse_modules(contents: &str) -> Vec<Module> {
    contents.lines()
        .map(|l| l.trim().parse::<i32>().unwrap())
        .map(|m| Module::new(m))
        .collect()
}

fn part1(modules: &Vec<Module>) -> i32 {
    modules.iter().map(|m| m.required_fuel()).sum()
}

fn part2(modules: &Vec<Module>) -> i32 {
    modules.iter().map(|m| m.recursive_fuel()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recursive_fuel_for_14_should_be_2() {
        let module = Module::new(14);

        assert_eq!(2, module.recursive_fuel());
    }
}
