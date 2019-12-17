use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::iter::FromIterator;

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

fn part_two(program: &Vec<i32>) -> Result<Option<i32>, String> {
    match compute(program, &vec![5]) {
        Ok((output, _)) => Ok(output),
        Err(s) => Err(s),
    }
}

fn calculate_signal(program: &Vec<i32>, phase_settings: &Vec<i32>) -> Result<i32, String> {
    let mut current_input = 0;

    for setting in phase_settings {
        let (output, _) = compute(program, &vec![setting.clone(), current_input])?;
        current_input = output.unwrap();
    }

    Ok(current_input)
}

fn calculate_signal_with_feedback(
    program: &Vec<i32>,
    phase_settings: &Vec<i32>,
) -> Result<i32, String> {
    let phase_length = phase_settings.len();
    let mut states = HashMap::new();

    // Initialize thrusters
    for i in 0..phase_length {
        let (_, state) = compute(&program, &vec![phase_settings[i]])?;
        states.insert(i, state.clone());
    }

    let mut current_input = 0;
    let mut index = 0;

    loop {
        let state = states.get(&index).unwrap();
        let (output, new_state) = compute(state, &vec![current_input])?;
        if let Some(o) = output {
            current_input = o;
            *states.get_mut(&index).unwrap() = new_state.clone();
            index = (index + 1) % phase_length;
        } else {
            return Ok(current_input);
        }
    }

    Ok(current_input)
}

fn compute(program: &Vec<i32>, inputs: &Vec<i32>) -> Result<(Option<i32>, Vec<i32>), String> {
    println!("{:?}", program);
    println!("{:?}", inputs);

    let mut inputs = inputs.iter().cycle();
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
            }
            Opcode::Multiply => {
                let first_parameter = get_parameter(i + 1, &first_mode, &program);
                let second_parameter = get_parameter(i + 2, &second_mode, &program);
                let result_index = program[i + 3] as usize;

                program[result_index] = first_parameter * second_parameter;

                i += 4;
            }
            Opcode::Set => {
                let result_index = program[i + 1] as usize;

                program[result_index] = inputs.next().unwrap().clone();

                i += 2;
            }
            Opcode::Output => {
                let value = match first_mode {
                    ParameterMode::Position => {
                        let index = program[i + 1] as usize;
                        program[index]
                    }
                    ParameterMode::Immediate => program[i + 1],
                };

                output = Some(value);

                i += 2;
            }
            Opcode::JumpIfTrue => {
                let first_parameter = get_parameter(i + 1, &first_mode, &program);
                let second_parameter = get_parameter(i + 2, &second_mode, &program);

                if first_parameter != 0 {
                    i = second_parameter as usize;
                } else {
                    i += 3;
                }
            }
            Opcode::JumpIfFalse => {
                let first_parameter = get_parameter(i + 1, &first_mode, &program);
                let second_parameter = get_parameter(i + 2, &second_mode, &program);

                if first_parameter == 0 {
                    i = second_parameter as usize;
                } else {
                    i += 3;
                }
            }
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
            }
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
            }
            Opcode::Halt => {
                break;
            }
        }
    }

    Ok((output, program))
}

fn parse_operation(
    operation: i32,
) -> Result<(Opcode, ParameterMode, ParameterMode, ParameterMode), String> {
    let opcode = parse_opcode(operation % 100)?;
    let first_parameter_mode = parse_mode(operation / 100 % 10)?;
    let second_parameter_mode = parse_mode(operation / 1000 % 10)?;
    let third_parameter_mode = parse_mode(operation / 10000 % 10)?;

    Ok((
        opcode,
        first_parameter_mode,
        second_parameter_mode,
        third_parameter_mode,
    ))
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
        _ => Err(format!("Invalid opcode: {}", opcode)),
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
        }
        ParameterMode::Immediate => program[index],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_operation_test() {
        let operation = 1002;
        assert_eq!(
            Ok((
                Opcode::Multiply,
                ParameterMode::Position,
                ParameterMode::Immediate,
                ParameterMode::Position
            )),
            parse_operation(operation)
        );
    }

    #[test]
    fn compute_test_1() {
        let program = vec![1101, 100, -1, 4, 0];
        let (_, result) = compute(&program, &vec![1]).unwrap();

        let expected = vec![1101, 100, -1, 4, 99];

        assert_eq!(expected, result);
    }

    #[test]
    fn compute_test_2() {
        let program = vec![1, 0, 0, 0, 99];
        let (_, result) = compute(&program, &vec![1]).unwrap();

        let expected = vec![2, 0, 0, 0, 99];

        assert_eq!(expected, result);
    }

    #[test]
    fn compute_test_3() {
        let program = vec![2, 3, 0, 3, 99];
        let (_, result) = compute(&program, &vec![1]).unwrap();

        let expected = vec![2, 3, 0, 6, 99];

        assert_eq!(expected, result);
    }

    #[test]
    fn compute_test_4() {
        let program = vec![2, 4, 4, 5, 99, 0];
        let (_, result) = compute(&program, &vec![1]).unwrap();

        let expected = vec![2, 4, 4, 5, 99, 9801];

        assert_eq!(expected, result);
    }

    #[test]
    fn compute_test_5() {
        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let (_, result) = compute(&program, &vec![1]).unwrap();

        let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];

        assert_eq!(expected, result);
    }

    #[test]
    fn compute_test_6() {
        let program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let (_, result) = compute(&program, &vec![1]).unwrap();

        let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

        assert_eq!(expected, result);
    }

    #[test]
    fn compute_test_7() {
        let program = vec![1002, 4, 3, 4, 33];
        let (_, result) = compute(&program, &vec![1]).unwrap();

        let expected = vec![1002, 4, 3, 4, 99];

        assert_eq!(expected, result);
    }

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

    #[test]
    fn signal_feedback_1() {
        let program = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let phase_settings = vec![9, 8, 7, 6, 5];

        assert_eq!(
            Ok(139629729),
            calculate_signal_with_feedback(&program, &phase_settings)
        );
    }
}
