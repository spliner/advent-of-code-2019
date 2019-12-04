use std::error::Error;
use std::fs;
use std::ops::RangeInclusive;

pub mod config;

type Password = String;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let range = parse_input(&contents);

    match config.part {
        config::Part::PartOne => {
            let count = part_one(range);
            println!("{}", count);
        }
        config::Part::PartTwo => {
            println!("Not yet!");
        }
    }

    Ok(())
}

fn parse_input(contents: &str) -> (i32, i32) {
    let mut parts = contents.trim().split("-").map(|s| s.trim().parse::<i32>().unwrap());
    let start = parts.next().unwrap();
    let end = parts.next().unwrap();

    (start, end)
}

fn part_one(range: (i32, i32)) -> usize {
    let (start, end) = range;
    RangeInclusive::new(start, end)
        .filter(|n| {
            let password = n.to_string();
            is_valid(&password)
        })
        .count()
}

fn is_valid(password: &Password) -> bool {
    if password.len() != 6 {
        return false;
    }

    let mut has_same_adjacent_digits = false;
    let mut never_decreases = true;

    let mut chars = password.chars();
    let mut previous_char = chars.next().unwrap();

    for current_char in chars {
        has_same_adjacent_digits = has_same_adjacent_digits || current_char == previous_char;
        never_decreases = never_decreases && current_char >= previous_char;
        previous_char = current_char;
    }

    has_same_adjacent_digits && never_decreases
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_example_one_should_be_true() {
        assert_eq!(true, is_valid(&String::from("111111")));
    }

    #[test]
    fn is_valid_example_two_should_be_true() {
        assert_eq!(false, is_valid(&String::from("223450")));
    }

    #[test]
    fn is_valid_example_three_should_be_true() {
        assert_eq!(false, is_valid(&String::from("123789")));
    }
}

