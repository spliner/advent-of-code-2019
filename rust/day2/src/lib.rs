
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
            let result = part2(&intcode, 19690720);
            println!("{:?}", result);
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
    let clone = intcode_with_parameters(intcode, 12, 2);
    let result = run_intcode(&clone);

    result[0]
}

fn intcode_with_parameters(intcode: &Vec<usize>, noun: usize, verb: usize) -> Vec<usize> {
    let mut clone = intcode.clone();

    if clone.len() > 1 {
        clone[1] = noun;
    }

    if clone.len() > 2 {
        clone[2] = verb;
    }

    clone
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

// TODO: Do it in parallel
fn part2(intcode: &Vec<usize>, expected_output: usize) -> Option<usize> {
    for noun in 0..100 {
        for verb in 0..100 {
            let clone = intcode_with_parameters(intcode, noun, verb);
            let result = run_intcode(&clone);

            if result[0] == expected_output {
                return Some(100 * noun + verb);
            }
        }
    }

    None
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
