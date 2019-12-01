use std::cmp::max;
use std::error::Error;
use std::fs;

pub mod config;

struct Module {
    mass: i32,
}

impl Module {
    pub fn new(mass: i32) -> Self {
        Self { mass }
    }

    pub fn required_fuel(&self) -> i32 {
        Self::get_required_fuel(self.mass)
    }

    fn get_required_fuel(mass: i32) -> i32 {
        max(mass / 3 - 2, 0)
    }

    pub fn recursive_fuel(&self) -> i32 {
        Self::get_recursive_fuel(self.mass)
    }

    fn get_recursive_fuel(mass: i32) -> i32 {
        if mass <= 0 {
            return 0;
        }

        let required_fuel = Self::get_required_fuel(mass);
        return required_fuel + Self::get_recursive_fuel(required_fuel);
    }
}

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let modules = parse_modules(&contents);

    match config.part {
        config::Part::Part1 => {
            let result = part1(&modules);
            println!("{}", result);
        }
        config::Part::Part2 => {
            let result = part2(&modules);
            println!("{}", result);
        }
    }

    Ok(())
}

fn parse_modules(contents: &str) -> Vec<Module> {
    contents
        .lines()
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
