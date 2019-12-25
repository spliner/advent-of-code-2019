use std::clone::Clone;

#[derive(Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
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
    SetRelativeBase,
    Halt,
}

#[derive(Debug)]
pub struct Intcode {
    halted: bool,
    initial_program: Vec<i64>,
    current_state: Vec<i64>,
    current_position: usize,
    relative_base: usize,
    input: Option<i64>,
    outputs: Vec<i64>,
}

impl Intcode {
    pub fn new(program: &Vec<i64>) -> Self {
        Self {
            halted: false,
            initial_program: program.clone(),
            current_state: program.clone(),
            current_position: 0,
            relative_base: 0,
            input: None,
            outputs: Vec::new(),
        }
    }

    pub fn halted(&self) -> bool {
        self.halted
    }

    pub fn set_input(&mut self, input: i64) {
        self.input = Some(input);
    }

    pub fn outputs(&self) -> &Vec<i64> {
        &self.outputs
    }

    pub fn last_output(&self) -> Option<i64> {
        if self.outputs.len() > 0 {
            Some(self.outputs[self.outputs.len() - 1])
        } else {
            None
        }
    }

    pub fn current_state(&self) -> &Vec<i64> {
        &self.current_state
    }

    fn get(&mut self, index: usize) -> i64 {
        self.ensure_index(index);

        self.current_state[index]
    }

    fn set(&mut self, index: usize, value: i64) {
        self.ensure_index(index);

        println!("Setting {} in {}", value, index);

        self.current_state[index] = value;
    }

    fn get_parameter(&mut self, index: usize, mode: &ParameterMode) -> i64 {
        let index = match mode {
            ParameterMode::Position => {
                self.get(index) as usize
            }
            ParameterMode::Relative => {
                let absolute_index = self.get(index);
                let base = self.relative_base as i64;
                let relative_index = absolute_index + base;
                relative_index as usize
            }
            ParameterMode::Immediate => index,
        };

        self.get(index)
    }

    fn ensure_index(&mut self, index: usize) {
        for _ in self.current_state.len()..index + 1 {
            self.current_state.push(0);
        }
    }

    fn get_result_index(&mut self, index: usize, mode: &ParameterMode) -> usize {
        match mode {
            ParameterMode::Relative => {
                let absolute_index = self.get(index);
                let relative_index = absolute_index + self.relative_base as i64;
                relative_index as usize
            }
            _ => self.get(index) as usize
        }
    }

    pub fn compute(&mut self) -> Result<(), String> {
        while self.current_position < self.current_state.len() && !self.halted {
            let operation = self.get(self.current_position);
            println!("Loop => {} ({})", operation, self.current_position);
            let (opcode, first_mode, second_mode, third_mode) = parse_operation(operation)?;

            match opcode {
                Opcode::Add => {
                    println!("Add");

                    let first_parameter = self.get_parameter(self.current_position + 1, &first_mode);
                    println!("Param 1: {} (from {})", first_parameter, self.current_position + 1);

                    let second_parameter = self.get_parameter(self.current_position + 2, &second_mode);
                    println!("Param 2: {} (from {})", second_parameter, self.current_position + 2);

                    let result_index = self.get_result_index(self.current_position + 3, &third_mode);
                    println!("Storing {} in {} (from {})", first_parameter + second_parameter, result_index, self.current_position + 3);
                    println!(" ");

                    if result_index < 0 {
                        panic!("noob");
                    }

                    self.set(result_index as usize, first_parameter + second_parameter);

                    self.current_position += 4;
                }
                Opcode::Multiply => {
                    let first_parameter = self.get_parameter(self.current_position + 1, &first_mode);
                    let second_parameter = self.get_parameter(self.current_position + 2, &second_mode);
                    let result_index = self.get_result_index(self.current_position + 3, &third_mode);

                    println!("Multiply");
                    println!("Param 1: {} (from {})", first_parameter, self.current_position + 1);
                    println!("Param 2: {} (from {})", second_parameter, self.current_position + 2);
                    println!("Storing {} in {} (from {})", first_parameter * second_parameter, result_index, self.current_position + 3);
                    println!(" ");

                    self.set(result_index as usize, first_parameter * second_parameter);

                    self.current_position += 4;
                }
                Opcode::Set => {
                    let result_index = self.get_result_index(self.current_position + 1, &first_mode);

                    match self.input {
                        Some(i) => {
                            println!("Setting {} in {}", i, result_index);
                            println!(" ");
                            self.set(result_index, i);
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
                            let index = self.get(self.current_position + 1) as usize;
                            self.get(index)
                        }
                        ParameterMode::Relative => {
                            let absolute_index = self.get(self.current_position + 1);
                            let index = absolute_index + self.relative_base as i64;

                            self.get(index as usize)
                        }
                        ParameterMode::Immediate => self.get(self.current_position + 1),
                    };

                    println!("Output");
                    println!("{}", value);
                    println!(" ");

                    self.outputs.push(value);

                    self.current_position += 2;
                }
                Opcode::JumpIfTrue => {
                    let first_parameter = self.get_parameter(self.current_position + 1, &first_mode);
                    let second_parameter = self.get_parameter(self.current_position + 2, &second_mode);

                    println!("JumpIfTrue");
                    println!("Param 1: {} (from {})", first_parameter, self.current_position + 1);
                    println!("Param 2: {} (from {})", second_parameter, self.current_position + 2);

                    if first_parameter != 0 {
                        self.current_position = second_parameter as usize;
                        println!("Jumping to: {}", self.current_position);
                    } else {
                        self.current_position += 3;
                        println!("New position: {}", self.current_position);
                    }

                    println!(" ");
                }
                Opcode::JumpIfFalse => {
                    println!("JumpIfFalse");
                    let first_parameter = self.get_parameter(self.current_position + 1, &first_mode);
                    println!("Param 1: {} (from {})", first_parameter, self.current_position + 1);

                    let second_parameter = self.get_parameter(self.current_position + 2, &second_mode);
                    println!("Param 2: {} (from {})", second_parameter, self.current_position + 2);

                    if first_parameter == 0 {
                        self.current_position = second_parameter as usize;
                    } else {
                        self.current_position += 3;
                    }

                    println!("New position: {}", self.current_position);
                    println!(" ");
                }
                Opcode::LessThan => {
                    let first_parameter = self.get_parameter(self.current_position + 1, &first_mode);
                    let second_parameter = self.get_parameter(self.current_position + 2, &second_mode);
                    let result_index = self.get_result_index(self.current_position + 3, &third_mode);

                    let value = if first_parameter < second_parameter {
                        1
                    } else {
                        0
                    };

                    println!("LessThan");
                    println!("Param 1: {} (from {})", first_parameter, self.current_position + 1);
                    println!("Param 2: {} (from {})", second_parameter, self.current_position + 2);
                    println!("Storing {} in {}", value, result_index);
                    println!(" ");

                    self.set(result_index, value);

                    self.current_position += 4;
                }
                Opcode::Equals => {
                    let first_parameter = self.get_parameter(self.current_position + 1, &first_mode);
                    let second_parameter = self.get_parameter(self.current_position + 2, &second_mode);
                    let result_index = self.get_result_index(self.current_position + 3, &third_mode);

                    let value = if first_parameter == second_parameter {
                        1
                    } else {
                        0
                    };

                    self.set(result_index, value);

                    println!("Equals");
                    println!("Param 1: {} (from {})", first_parameter, self.current_position + 1);
                    println!("Param 2: {} (from {})", second_parameter, self.current_position + 2);
                    println!("Storing {} in {}", value, result_index);
                    println!(" ");

                    self.current_position += 4;
                }
                Opcode::SetRelativeBase => {
                    println!("SetRelativeBase");
                    let base_adjustment = self.get_parameter(self.current_position + 1, &first_mode);
                    println!("Adding {} to relative base (from {})", base_adjustment, self.current_position + 1);

                    let current_base = self.relative_base as i64;
                    let new_base = current_base + base_adjustment;

                    self.relative_base = new_base as usize;
                    println!("New base: {}", self.relative_base);
                    println!(" ");

                    self.current_position += 2;
                }
                Opcode::Halt => {
                    println!("Halt");
                    self.halted = true;
                }
            };
        }

        Ok(())
    }
}

fn parse_operation(
    operation: i64,
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

fn parse_opcode(opcode: i64) -> Result<Opcode, String> {
    match opcode % 100 {
        1 => Ok(Opcode::Add),
        2 => Ok(Opcode::Multiply),
        3 => Ok(Opcode::Set),
        4 => Ok(Opcode::Output),
        5 => Ok(Opcode::JumpIfTrue),
        6 => Ok(Opcode::JumpIfFalse),
        7 => Ok(Opcode::LessThan),
        8 => Ok(Opcode::Equals),
        9 => Ok(Opcode::SetRelativeBase),
        99 => Ok(Opcode::Halt),
        _ => Err(format!("Invalid opcode: {}", opcode)),
    }
}

fn parse_mode(mode: i64) -> Result<ParameterMode, String> {
    match mode {
        0 => Ok(ParameterMode::Position),
        1 => Ok(ParameterMode::Immediate),
        2 => Ok(ParameterMode::Relative),
        _ => Err(format!("Invalid param mode: {}", mode)),
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

    #[test]
    fn relative_mode_test_1() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut intcode = Intcode::new(&program);
        intcode.compute().unwrap();

        assert_eq!(&program, intcode.outputs());
    }

    #[test]
    fn relative_mode_test_2() {
        let program = vec![104i64, 1125899906842624, 99];
        let mut intcode = Intcode::new(&program);
        intcode.compute().unwrap();

        assert_eq!(1125899906842624, intcode.last_output().unwrap());
    }

    #[test]
    fn relative_mode_test_3() {
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut intcode = Intcode::new(&program);
        intcode.compute().unwrap();

        assert_eq!(16, intcode.last_output().unwrap().to_string().len());
    }
}
