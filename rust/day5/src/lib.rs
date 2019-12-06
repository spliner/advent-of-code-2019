use std::error::Error;
use std::fs;

pub mod config;

#[derive(Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

#[derive(Debug, PartialEq)]
enum Opcode {
    Add,
    Multiply,
    Set,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

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

fn part_one(program: &Vec<i32>) -> Result<Option<i32>, String> {
    match compute(program, 1) {
        Ok((output, _)) => Ok(output),
        Err(s) => Err(s),
    }
}

fn part_two(program: &Vec<i32>) -> Result<Option<i32>, String> {
    match compute(program, 5) {
        Ok((output, _)) => Ok(output),
        Err(s) => Err(s),
    }
}

fn compute(program: &Vec<i32>, input: i32) -> Result<(Option<i32>, Vec<i32>), String> {
    let mut program = program.clone();
    let mut i = 0;
    let mut output = None;

    while i < program.len() {
        let operation = program[i];
        let (opcode, first_mode, second_mode, _third_mode) = parse_operation(operation)?;

        match opcode {
            Opcode::Add => {
                let first_parameter = get_parameter(i + 1, &first_mode, &program);
                let second_parameter = get_parameter(i + 2, &second_mode, &program);
                let result_index = program[i + 3] as usize;

                program[result_index] = first_parameter + second_parameter;

                i += 4;
            },
            Opcode::Multiply => {
                let first_parameter = get_parameter(i + 1, &first_mode, &program);
                let second_parameter = get_parameter(i + 2, &second_mode, &program);
                let result_index = program[i + 3] as usize;

                program[result_index] = first_parameter * second_parameter;

                i += 4;
            },
            Opcode::Set => {
                let result_index = program[i + 1] as usize;

                program[result_index] = input;

                i += 2;
            },
            Opcode::Output => {
                let value = match first_mode {
                    ParameterMode::Position => {
                        let index = program[i + 1] as usize;
                        program[index]
                    },
                    ParameterMode::Immediate => {
                        program[i + 1]
                    }
                };

                output = Some(value);

                i += 2;
            },
            Opcode::JumpIfTrue => {
                let first_parameter = get_parameter(i + 1, &first_mode, &program);
                let second_parameter = get_parameter(i + 2, &second_mode, &program);

                if first_parameter != 0 {
                    i = second_parameter as usize;
                } else {
                    i += 3;
                }
            },
            Opcode::JumpIfFalse => {
                let first_parameter = get_parameter(i + 1, &first_mode, &program);
                let second_parameter = get_parameter(i + 2, &second_mode, &program);

                if first_parameter == 0 {
                    i = second_parameter as usize;
                } else {
                    i += 3;
                }
            },
            Opcode::LessThan => {
                let first_parameter = get_parameter(i + 1, &first_mode, &program);
                let second_parameter = get_parameter(i + 2, &second_mode, &program);
                let result_index = program[i + 3] as usize;

                let value = if first_parameter < second_parameter {
                    1
                } else {
                    0
                };

                program[result_index] = value;

                i += 4;
            },
            Opcode::Equals => {
                let first_parameter = get_parameter(i + 1, &first_mode, &program);
                let second_parameter = get_parameter(i + 2, &second_mode, &program);
                let result_index = program[i + 3] as usize;

                let value = if first_parameter == second_parameter {
                    1
                } else {
                    0
                };

                program[result_index] = value;

                i += 4;
            },
            Opcode::Halt => {
                break;
            },
        }
    }

    Ok((output, program))
}

fn parse_operation(operation: i32) -> Result<(Opcode, ParameterMode, ParameterMode, ParameterMode), String> {
    let opcode = parse_opcode(operation % 100)?;
    let first_parameter_mode = parse_mode(operation / 100 % 10)?;
    let second_parameter_mode = parse_mode(operation / 1000 % 10)?;
    let third_parameter_mode = parse_mode(operation / 10000 % 10)?;

    Ok((opcode, first_parameter_mode, second_parameter_mode, third_parameter_mode))
}

fn parse_opcode(opcode: i32) -> Result<Opcode, String> {
    match opcode % 100 {
        1 => Ok(Opcode::Add),
        2 => Ok(Opcode::Multiply),
        3 => Ok(Opcode::Set),
        4 => Ok(Opcode::Output),
        5 => Ok(Opcode::JumpIfTrue),
        6 => Ok(Opcode::JumpIfFalse),
        7 => Ok(Opcode::LessThan),
        8 => Ok(Opcode::Equals),
        99 => Ok(Opcode::Halt),
        _ => Err(format!("Invalid opcode: {}", opcode))
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
            Ok((Opcode::Multiply, ParameterMode::Position, ParameterMode::Immediate, ParameterMode::Position)),
            parse_operation(operation));
    }

    #[test]
    fn compute_test_1() {
        let program = vec![1101, 100, -1, 4, 0];
        let (_, result) = compute(&program, 1).unwrap();

        let expected = vec![1101, 100, -1, 4, 99];

        assert_eq!(expected, result);
    }

    #[test]
    fn compute_test_2() {
        let program = vec![1, 0, 0, 0, 99];
        let (_, result) = compute(&program, 1).unwrap();

        let expected = vec![2, 0, 0, 0, 99];

        assert_eq!(expected, result);
    }

    #[test]
    fn compute_test_3() {
        let program = vec![2, 3, 0, 3, 99];
        let (_, result) = compute(&program, 1).unwrap();

        let expected = vec![2, 3, 0, 6, 99];

        assert_eq!(expected, result);
    }

    #[test]
    fn compute_test_4() {
        let program = vec![2, 4, 4, 5, 99, 0];
        let (_, result) = compute(&program, 1).unwrap();

        let expected = vec![2, 4, 4, 5, 99, 9801];

        assert_eq!(expected, result);
    }

    #[test]
    fn compute_test_5() {
        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let (_, result) = compute(&program, 1).unwrap();

        let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];

        assert_eq!(expected, result);
    }

    #[test]
    fn compute_test_6() {
        let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let (_, result) = compute(&program, 1).unwrap();

        let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

        assert_eq!(expected, result);
    }

    #[test]
    fn compute_test_7() {
        let program = vec![1002, 4, 3, 4, 33];
        let (_, result) = compute(&program, 1).unwrap();

        let expected = vec![1002, 4, 3, 4, 99];

        assert_eq!(expected, result);
    }
}
