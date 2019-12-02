
use std::error::Error;
use std::fs;

pub mod config;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let intcode = parse_intcode(&contents);

    match config.part {
        config::Part::Part1 => {
            let result = part1(&intcode);
            println!("{}", result);
        }
        config::Part::Part2 => {
            // TODO: Part 2
        }
    }

    Ok(())
}

fn parse_intcode(contents: &str) -> Vec<usize> {
    contents
        .split(",")
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect()
}

fn part1(intcode: &Vec<usize>) -> usize {
    let mut cloned = intcode.clone();
    if intcode.len() > 1 {
        cloned[1] = 12;
    }

    if intcode.len() > 2 {
        cloned[2] = 2;
    }

    let result = run_intcode(&cloned);

    result[0]
}

fn run_intcode(intcode: &Vec<usize>) -> Vec<usize> {
    let mut intcode = intcode.clone();

    let get_indexes = |i: usize, intcode: &Vec<usize>| {
        let x_index = intcode[i + 1];
        let y_index = intcode[i + 2];
        let result_index = intcode[i + 3];

        (x_index, y_index, result_index)
    };

    let mut i = 0;
    loop {
        let value = intcode[i];
        match value {
            1 => {
                let (x_index, y_index, result_index) = get_indexes(i, &intcode);
                intcode[result_index] = intcode[x_index] + intcode[y_index];
                i += 4;
            },
            2 => {
                let (x_index, y_index, result_index) = get_indexes(i, &intcode);
                intcode[result_index] = intcode[x_index] * intcode[y_index];
                i += 4;
            },
            99 => {
                break;
            },
            _ => {
                i += 1;
            }
        };
    }

    intcode
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let contents = "1,2,3,4, 5";

        let expected = vec![1usize, 2, 3, 4, 5];

        assert_eq!(expected, parse_intcode(&contents));
    }

    #[test]
    fn run_intcode_example1() {
        let intcode = vec![1usize, 0usize, 0usize, 0usize, 99usize];

        let expected = vec![2usize, 0, 0, 0, 99];

        assert_eq!(expected, run_intcode(&intcode));
    }

    #[test]
    fn run_intcode_example2() {
        let intcode = vec![2usize, 3, 0, 3, 99];

        let expected = vec![2usize, 3, 0, 6, 99];

        assert_eq!(expected, run_intcode(&intcode));
    }

    #[test]
    fn run_intcode_example3() {
        let intcode = vec![2usize, 4, 4, 5, 99, 0];

        let expected = vec![2usize, 4, 4, 5, 99, 9801];

        assert_eq!(expected, run_intcode(&intcode));
    }

    #[test]
    fn run_intcode_example4() {
        let intcode = vec![1usize, 1, 1, 4, 99, 5, 6, 0, 99];

        let expected = vec![30usize, 1, 1, 4, 2, 5, 6, 0, 99];

        assert_eq!(expected, run_intcode(&intcode));
    }

    #[test]
    fn run_intcode_example5() {
        let intcode = vec![1usize, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

        assert_eq!(expected, run_intcode(&intcode));
    }
}
