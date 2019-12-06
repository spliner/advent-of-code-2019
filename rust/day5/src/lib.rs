use std::error::Error;
use std::fs;

pub mod config;

enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    fn parse(value: i32) -> Result<ParameterMode, String> {
        match value as usize {
            0 => Ok(ParameterMode::Position),
            1 => Ok(ParameterMode::Immediate),
            _ => Err(format!("Invalid parameter mode: {}", value)),
        }
    }

    fn effective_value(&self, value: usize, instructions: &Vec<i32>) -> i32 {
        match self {
            ParameterMode::Position => {
                let position = instructions[value] as usize;
                instructions[position]
            },
            ParameterMode::Immediate => {
                instructions[value]
            },
        }
    }
}

trait Instruction {
    fn execute(&self, state: &mut ProgramExecutionState) -> Option<i32>;
    fn next_instruction_position(&self, index: usize) -> Option<usize>;
}

struct AddInstruction {
    parameter_one_mode: ParameterMode,
    parameter_two_mode: ParameterMode,
}

impl Instruction for AddInstruction {
    fn execute(&self, state: &mut ProgramExecutionState) -> Option<i32> {
        let a = self.parameter_one_mode
            .effective_value(
                state.index + 1,
                &state.instructions);
        let b = self.parameter_two_mode
            .effective_value(
                state.index + 2,
                &state.instructions);

        let index = state.instructions[state.index + 3] as usize;
        println!("{} + {} = {} in {}", a, b, a + b, index);
        state.instructions[index] = a + b;

        None
    }

    fn next_instruction_position(&self, index: usize) -> Option<usize> {
        Some(index + 4)
    }
}

struct MultiplyInstruction {
    parameter_one_mode: ParameterMode,
    parameter_two_mode: ParameterMode,
}

impl Instruction for MultiplyInstruction {
    fn execute(&self, state: &mut ProgramExecutionState) -> Option<i32> {
        let a = self.parameter_one_mode
            .effective_value(
                state.index + 1,
                &state.instructions);
        let b = self.parameter_two_mode
            .effective_value(
                state.index + 2,
                &state.instructions);

        let index = state.instructions[state.index + 3] as usize;
        println!("{} * {} = {} in {}", a, b, a * b, index);
        state.instructions[index] = a * b;

        None
    }

    fn next_instruction_position(&self, index: usize) -> Option<usize> {
        Some(index + 4)
    }
}

struct SetInstruction {}

impl Instruction for SetInstruction {
    fn execute(&self, state: &mut ProgramExecutionState) -> Option<i32> {
        let address = state.instructions[state.index + 1] as usize;

        println!("{} in {}", state.input, address);

        state.instructions[address] = state.input;

        None
    }

    fn next_instruction_position(&self, index: usize) -> Option<usize> {
        Some(index + 2)
    }
}

struct OutputInstruction {
    parameter_mode: ParameterMode,
}

impl Instruction for OutputInstruction {
    fn execute(&self, state: &mut ProgramExecutionState) -> Option<i32> {
        let address = self.parameter_mode
            .effective_value(
                state.index + 1,
                &state.instructions);

        Some(state.instructions[address as usize])
    }

    fn next_instruction_position(&self, index: usize) -> Option<usize> {
        Some(index + 2)
    }
}

struct HaltInstruction {
}

impl Instruction for HaltInstruction {
    fn execute(&self, _state: &mut ProgramExecutionState) -> Option<i32> {
        None
    }

    fn next_instruction_position(&self, _index: usize) -> Option<usize> {
        None
    }
}

trait Program {
    fn input(&self) -> i32;
    fn instructions(&self) -> &Vec<i32>;
    fn execute(&self) -> ExecutionResult;
}

struct TestProgram {
    input: i32,
    instructions: Vec<i32>,
}

impl TestProgram {
    fn new(input: i32, instructions: Vec<i32>) -> Self {
        Self {
            input,
            instructions
        }
    }
}

impl Program for TestProgram {
    fn input(&self) -> i32 {
        self.input
    }

    fn instructions(&self) -> &Vec<i32> {
        &self.instructions
    }

    fn execute(&self) -> ExecutionResult {
        let state = ProgramExecutionState {
            index: 0,
            input: self.input,
            instructions: self.instructions.clone(),
        };

        let mut outputs = Vec::new();
        let mut foo = Vec::new();
        state
            .for_each(|(state, output)|  {
                if let Some(o) = output {
                    outputs.push(o);
                }

                foo = state;
            });

        ExecutionResult {
            outputs,
            final_state: foo,
        }
    }
}

struct ProgramExecutionState {
    index: usize,
    input: i32,
    instructions: Vec<i32>,
}

impl Iterator for ProgramExecutionState {
    type Item = (Vec<i32>, Option<i32>);

    fn next(&mut self) -> Option<Self::Item> {
        let raw_instruction = self.instructions[self.index];
        println!("Instruction {} at {}", raw_instruction, self.index);
        let opcode = raw_instruction % 100;
        let parameter_one_mode = ParameterMode::parse(raw_instruction / 100 % 10).unwrap();
        let parameter_two_mode = ParameterMode::parse(raw_instruction / 1000 % 10).unwrap();
        let _parameter_three_mode = ParameterMode::parse(raw_instruction / 10000 % 10).unwrap();

        let instruction: Box<dyn Instruction> = match opcode {
            1 => {
                Box::new(AddInstruction {
                    parameter_one_mode,
                    parameter_two_mode,
                })
            },
            2 => {
                Box::new(MultiplyInstruction {
                    parameter_one_mode,
                    parameter_two_mode,
                })
            },
            3 => {
                Box::new(SetInstruction {})
            },
            4 => {
                Box::new(OutputInstruction {
                    parameter_mode: parameter_one_mode,
                })
            },
            99 => {
                Box::new(HaltInstruction {})
            }
            _ => {
                // TODO: Don't actually panic
                panic!("This shouldn't happen")
            },
        };

        match instruction.next_instruction_position(self.index) {
            Some(i) => {
                let output = instruction.execute(self);
                self.index = i;
                Some((self.instructions.clone(), output))
            },
            None => None
        }
    }
}

#[derive(Debug, PartialEq)]
struct ExecutionResult {
    outputs: Vec<i32>,
    final_state: Vec<i32>,
}

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let instructions: Vec<i32> = fs::read_to_string(config.filename)?
        .split(",")
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();
    let program = TestProgram::new(1, instructions);

    match config.part {
        config::Part::PartOne => {
            let output = program.execute();
            println!("{:?}", output);
        }
        config::Part::PartTwo => {
            // TODO: Part two
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{Program, TestProgram};

    #[test]
    fn execute_program_example_one() {
        let instructions = vec![1101, 100, -1, 4, 0];
        let program = TestProgram::new(1, instructions);
        let result = program.execute().final_state;

        let expected = vec![1101, 100, -1, 4, 99];

        assert_eq!(expected, result);
    }

    #[test]
    fn execute_program_negative_value() {
        let instructions = vec![1101, -100, -1, 5, 99, 1];
        let program = TestProgram::new(1, instructions);
        let result = program.execute().final_state;

        let expected = vec![1101, -100, -1, 5, 99, -101];

        assert_eq!(expected, result);
    }
}
