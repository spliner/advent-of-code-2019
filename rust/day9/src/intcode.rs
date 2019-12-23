use std::clone::Clone;

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

#[derive(Debug)]
pub struct Intcode {
    halted: bool,
    initial_program: Vec<i32>,
    current_state: Vec<i32>,
    current_position: usize,
    pub input: Option<i32>,
    outputs: Vec<i32>,
}

impl Intcode {
    pub fn new(program: &Vec<i32>) -> Self {
        Self {
            halted: false,
            initial_program: program.clone(),
            current_state: program.clone(),
            current_position: 0,
            input: None,
            outputs: Vec::new(),
        }
    }

    pub fn halted(&self) -> bool {
        self.halted
    }

    pub fn set_input(&mut self, input: i32) {
        self.input = Some(input);
    }

    pub fn outputs(&self) -> &Vec<i32> {
        &self.outputs
    }

    pub fn last_output(&self) -> Option<i32> {
        if self.outputs.len() > 0 {
            Some(self.outputs[self.outputs.len() - 1])
        } else {
            None
        }
    }

    pub fn current_state(&self) -> &Vec<i32> {
        &self.current_state
    }

    pub fn compute(&mut self) -> Result<(), String> {
        while self.current_position < self.current_state.len() && !self.halted {
            let operation = self.current_state[self.current_position];
            let (opcode, first_mode, second_mode, _third_mode) = parse_operation(operation)?;

            match opcode {
                Opcode::Add => {
                    let first_parameter =
                        get_parameter(self.current_position + 1, &first_mode, &self.current_state);
                    let second_parameter =
                        get_parameter(self.current_position + 2, &second_mode, &self.current_state);
                    let result_index = self.current_state[self.current_position + 3] as usize;

                    self.current_state[result_index] = first_parameter + second_parameter;

                    self.current_position += 4;
                }
                Opcode::Multiply => {
                    let first_parameter =
                        get_parameter(self.current_position + 1, &first_mode, &self.current_state);
                    let second_parameter =
                        get_parameter(self.current_position + 2, &second_mode, &self.current_state);
                    let result_index = self.current_state[self.current_position + 3] as usize;

                    self.current_state[result_index] = first_parameter * second_parameter;

                    self.current_position += 4;
                }
                Opcode::Set => {
                    let result_index = self.current_state[self.current_position + 1] as usize;

                    match self.input {
                        Some(i) => {
                            self.current_state[result_index] = i;
                            self.input = None;
                            self.current_position += 2;
                        }
                        None => {
                            break;
                        }
                    }
                }
                Opcode::Output => {
                    let value = match first_mode {
                        ParameterMode::Position => {
                            let index = self.current_state[self.current_position + 1] as usize;
                            self.current_state[index]
                        }
                        ParameterMode::Immediate => self.current_state[self.current_position + 1],
                    };

                    self.outputs.push(value);

                    self.current_position += 2;
                }
                Opcode::JumpIfTrue => {
                    let first_parameter =
                        get_parameter(self.current_position + 1, &first_mode, &self.current_state);
                    let second_parameter =
                        get_parameter(self.current_position + 2, &second_mode, &self.current_state);

                    if first_parameter != 0 {
                        self.current_position = second_parameter as usize;
                    } else {
                        self.current_position += 3;
                    }
                }
                Opcode::JumpIfFalse => {
                    let first_parameter =
                        get_parameter(self.current_position + 1, &first_mode, &self.current_state);
                    let second_parameter =
                        get_parameter(self.current_position + 2, &second_mode, &self.current_state);

                    if first_parameter == 0 {
                        self.current_position = second_parameter as usize;
                    } else {
                        self.current_position += 3;
                    }
                }
                Opcode::LessThan => {
                    let first_parameter =
                        get_parameter(self.current_position + 1, &first_mode, &self.current_state);
                    let second_parameter =
                        get_parameter(self.current_position + 2, &second_mode, &self.current_state);
                    let result_index = self.current_state[self.current_position + 3] as usize;

                    let value = if first_parameter < second_parameter {
                        1
                    } else {
                        0
                    };

                    self.current_state[result_index] = value;

                    self.current_position += 4;
                }
                Opcode::Equals => {
                    let first_parameter =
                        get_parameter(self.current_position + 1, &first_mode, &self.current_state);
                    let second_parameter =
                        get_parameter(self.current_position + 2, &second_mode, &self.current_state);
                    let result_index = self.current_state[self.current_position + 3] as usize;

                    let value = if first_parameter == second_parameter {
                        1
                    } else {
                        0
                    };

                    self.current_state[result_index] = value;

                    self.current_position += 4;
                }
                Opcode::Halt => {
                    self.halted = true;
                }
            };
        }

        Ok(())
    }
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
        let mut intcode = Intcode::new(&vec![1101, 100, -1, 4, 0]);
        intcode.set_input(1);
        intcode.compute().unwrap();

        let expected = vec![1101, 100, -1, 4, 99];

        assert_eq!(&expected, intcode.current_state());
    }

    #[test]
    fn compute_test_2() {
        let mut intcode = Intcode::new(&vec![1, 0, 0, 0, 99]);
        intcode.set_input(1);
        intcode.compute().unwrap();

        let expected = vec![2, 0, 0, 0, 99];

        assert_eq!(&expected, intcode.current_state());
    }

    #[test]
    fn compute_test_3() {
        let mut intcode = Intcode::new(&vec![2, 3, 0, 3, 99]);
        intcode.set_input(1);
        intcode.compute().unwrap();

        let expected = vec![2, 3, 0, 6, 99];

        assert_eq!(&expected, intcode.current_state());
    }

    #[test]
    fn compute_test_4() {
        let mut intcode = Intcode::new(&vec![2, 4, 4, 5, 99, 0]);
        intcode.set_input(1);
        intcode.compute().unwrap();

        let expected = vec![2, 4, 4, 5, 99, 9801];

        assert_eq!(&expected, intcode.current_state());
    }

    #[test]
    fn compute_test_5() {
        let mut intcode = Intcode::new(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        intcode.set_input(1);
        intcode.compute().unwrap();

        let expected = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];

        assert_eq!(&expected, intcode.current_state());
    }

    #[test]
    fn compute_test_6() {
        let mut intcode = Intcode::new(&vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        intcode.set_input(1);
        intcode.compute().unwrap();

        let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

        assert_eq!(&expected, intcode.current_state());
    }

    #[test]
    fn compute_test_7() {
        let mut intcode = Intcode::new(&vec![1002, 4, 3, 4, 33]);
        intcode.set_input(1);
        intcode.compute().unwrap();

        let expected = vec![1002, 4, 3, 4, 99];

        assert_eq!(&expected, intcode.current_state());
    }
}
