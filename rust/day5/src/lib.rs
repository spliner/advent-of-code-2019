use std::error::Error;
use std::fs;

pub mod config;

#[derive(Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Add,
    Multiply,
    Set,
    Output,
    Halt,
}

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let program: Vec<i32> = fs::read_to_string(config.filename)?
        .split(",")
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();

    match config.part {
        config::Part::PartOne => {
            part_one(&program, 1)?;
        }
        config::Part::PartTwo => {
            // TODO: Part two
        }
    }

    Ok(())
}

fn part_one(program: &Vec<i32>, input: i32) -> Result<Vec<i32>, String> {
    let mut program = program.clone();
    let mut i = 0;

    while i < program.len() {
        println!("{}", i);
        let operation = program[i];
        let (instruction, first_mode, second_mode) = parse_operation(operation)?;

        match instruction {
            Instruction::Add => {
                let first_parameter = get_parameter(i + 1, &first_mode, &program);
                let second_parameter = get_parameter(i + 2, &second_mode, &program);
                let result_index = program[i + 3] as usize;

                println!("{} + {} in {}", first_parameter, second_parameter, result_index);

                program[result_index] = first_parameter + second_parameter;

                i += 4;
            },
            Instruction::Multiply => {
                let first_parameter = get_parameter(i + 1, &first_mode, &program);
                let second_parameter = get_parameter(i + 2, &second_mode, &program);
                let result_index = program[i + 3] as usize;

                println!("{} * {} in {}", first_parameter, second_parameter, result_index);

                program[result_index] = first_parameter * second_parameter;

                i += 4;
            },
            Instruction::Set => {
                let result_index = program[i + 1] as usize;
                println!("Set {}", result_index);
                program[result_index] = input;
                i += 2;
            },
            Instruction::Output => {
                let value = match first_mode {
                    ParameterMode::Position => {
                        let index = program[i + 1] as usize;
                        program[index]
                    },
                    ParameterMode::Immediate => {
                        program[i + 1]
                    }
                };

                println!("Output: {}", value);

                i += 2;
            },
            Instruction::Halt => {
                break;
            },
        }
    }

    Ok(program)
}

fn parse_operation(operation: i32) -> Result<(Instruction, ParameterMode, ParameterMode), String> {
    let instruction = parse_instruction(operation % 100)?;
    let first_parameter_mode = parse_mode(operation / 100 % 10)?;
    let second_parameter_mode = parse_mode(operation / 1000 % 10)?;

    Ok((instruction, first_parameter_mode, second_parameter_mode))
}

fn parse_instruction(instruction: i32) -> Result<Instruction, String> {
    match instruction % 100 {
        1 => Ok(Instruction::Add),
        2 => Ok(Instruction::Multiply),
        3 => Ok(Instruction::Set),
        4 => Ok(Instruction::Output),
        99 => Ok(Instruction::Halt),
        _ => Err(format!("Invalid instruction: {}", instruction))
    }
}

fn parse_mode(mode: i32) -> Result<ParameterMode, String> {
    match mode {
        0 => Ok(ParameterMode::Position),
        1 => Ok(ParameterMode::Immediate),
        _ => Err(format!("Invalid param mode: {}", mode)),
    }
}

fn get_parameter(index: usize, mode: &ParameterMode, program: &Vec<i32>) -> i32 {
    match mode {
        ParameterMode::Position => {
            let index = program[index] as usize;
            program[index]
        },
        ParameterMode::Immediate => {
            program[index]
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_operation_test() {
        let operation = 1002;
        assert_eq!(
            Ok((Instruction::Multiply, ParameterMode::Position, ParameterMode::Immediate)),
            parse_operation(operation));
    }

    #[test]
    fn part_one_1() {
        let program = vec![1101, 100, -1, 4, 0];

        let expected = vec![1101, 100, -1, 4, 99];

        assert_eq!(Ok(expected), part_one(&program, 1));
    }

    #[test]
    fn part_one_2() {
        let program = vec![1, 0, 0, 0, 99];

        let expected = vec![2, 0, 0, 0, 99];

        assert_eq!(Ok(expected), part_one(&program, 1));
    }

    #[test]
    fn part_one_3() {
        let program = vec![2, 3, 0, 3, 99];

        let expected = vec![2, 3, 0, 6, 99];

        assert_eq!(Ok(expected), part_one(&program, 1));
    }

    #[test]
    fn part_one_4() {
        let program = vec![2, 4, 4, 5, 99, 0];

        let expected = vec![2, 4, 4, 5, 99, 9801];

        assert_eq!(Ok(expected), part_one(&program, 1));
    }

    #[test]
    fn part_one_5() {
        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];

        let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];

        assert_eq!(Ok(expected), part_one(&program, 1));
    }

    #[test]
    fn part_one_6() {
        let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];

        let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

        assert_eq!(Ok(expected), part_one(&program, 1));
    }

    #[test]
    fn part_one_7() {
        let program = vec![1002, 4, 3, 4, 33];

        let expected = vec![1002, 4, 3, 4, 99];

        assert_eq!(Ok(expected), part_one(&program, 1));
    }
}
