use std::error::Error;
use std::fs;
use std::ops::RangeInclusive;
use std::collections::HashMap;

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
            let count = part_two(range);
            println!("{}", count);
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
            is_valid(&password, false)
        })
        .count()
}

fn part_two(range: (i32, i32)) -> usize {
    let (start, end) = range;
    RangeInclusive::new(start, end)
        .filter(|n| {
            let password = n.to_string();
            is_valid(&password, true)
        })
        .count()
}

fn is_valid(password: &Password, must_be_two: bool) -> bool {
    if password.len() != 6 {
        return false;
    }

    let mut chars = password.chars();
    let mut previous_char = chars.next().unwrap();

    let mut char_counts = HashMap::new();
    let mut increase_count = |c| {
        let count = char_counts.entry(c).or_insert(0);
        *count += 1;
    };

    increase_count(previous_char);

    for current_char in chars {
        if current_char < previous_char {
            return false;
        }

        increase_count(current_char);

        previous_char = current_char;
    }

    char_counts.iter()
        .any(|(_, v)| {
            if must_be_two {
                v == &2
            } else {
                v > &1
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_part_one_example_one_should_be_true() {
        assert_eq!(true, is_valid(&String::from("111111"), false));
    }

    #[test]
    fn is_valid_part_one_example_two_should_be_false() {
        assert_eq!(false, is_valid(&String::from("223450"), false));
    }

    #[test]
    fn is_valid_part_one_example_three_should_be_false() {
        assert_eq!(false, is_valid(&String::from("123789"), false));
    }

    #[test]
    fn is_valid_part_two_example_one_should_be_true() {
        assert_eq!(true, is_valid(&String::from("112233"), true));
    }

    #[test]
    fn is_valid_part_two_example_two_should_be_false() {
        assert_eq!(false, is_valid(&String::from("123444"), true));
    }

    #[test]
    fn is_valid_part_two_example_three_should_be_true() {
        assert_eq!(true, is_valid(&String::from("111122"), true));
    }
}

