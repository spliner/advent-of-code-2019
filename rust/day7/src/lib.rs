use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::iter::FromIterator;

use crate::intcode::Intcode;

pub mod config;
pub mod intcode;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let program: Vec<i32> = fs::read_to_string(config.filename)?
        .split(",")
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();

    match config.part {
        config::Part::PartOne => {
            let result = part_one(&program)?;
            println!("{:?}", result);
        }
        config::Part::PartTwo => {
            let result = part_two(&program)?;
            println!("{:?}", result);
        }
    }

    Ok(())
}

fn part_one(program: &Vec<i32>) -> Result<i32, String> {
    let mut max_signal = 0;

    for a in 0..=4 {
        for b in 0..=4 {
            for c in 0..=4 {
                for d in 0..=4 {
                    for e in 0..=4 {
                        let phase_settings = vec![a, b, c, d, e];
                        // Ensure values are distinct
                        // TODO: Surely there's a better way to do this
                        let phase_set: HashSet<&i32> = HashSet::from_iter(phase_settings.iter());
                        if phase_set.len() != phase_settings.len() {
                            continue;
                        }

                        let result = calculate_signal(program, &phase_settings)?;
                        if result > max_signal {
                            max_signal = result;
                        }
                    }
                }
            }
        }
    }

    Ok(max_signal)
}

// TODO: Part 2
fn part_two(_program: &Vec<i32>) -> Result<Option<i32>, String> {
    Ok(None)
}

fn calculate_signal(program: &Vec<i32>, phase_settings: &Vec<i32>) -> Result<i32, String> {
    let mut current_input = 0;

    for setting in phase_settings {
        let mut intcode = Intcode::new(program);

        intcode.set_input(setting.clone());

        while !intcode.halted() {
            intcode.compute()?;
            intcode.set_input(current_input);
        }

        current_input = match intcode.last_output() {
            Some(o) => o,
            None => current_input,
        };
    }

    Ok(current_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signal_1() {
        let program = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let phase_settings = vec![4, 3, 2, 1, 0];

        assert_eq!(Ok(43210), calculate_signal(&program, &phase_settings));
    }

    #[test]
    fn part_one_example_one_should_return_43210() {
        let program = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];

        assert_eq!(Ok(43210), part_one(&program));
    }

    #[test]
    fn signal_2() {
        let program = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let phase_settings = vec![0, 1, 2, 3, 4];

        assert_eq!(Ok(54321), calculate_signal(&program, &phase_settings));
    }

    #[test]
    fn part_one_example_two_should_return_54321() {
        let program = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];

        assert_eq!(Ok(54321), part_one(&program));
    }

    #[test]
    fn signal_3() {
        let program = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let phase_settings = vec![1, 0, 4, 3, 2];

        assert_eq!(Ok(65210), calculate_signal(&program, &phase_settings));
    }

    #[test]
    fn part_one_example_three_should_return_65210() {
        let program = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];

        assert_eq!(Ok(65210), part_one(&program));
    }
}
