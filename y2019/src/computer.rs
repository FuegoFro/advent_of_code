use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::collections::{HashMap, VecDeque};
use std::mem;

pub type Word = i64;

#[derive(FromPrimitive, Clone, Debug)]
enum ParameterMode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

#[derive(Debug)]
struct Parameter {
    mode: ParameterMode,
    value: Word,
}

#[derive(Clone)]
struct Operation {
    num_parameters: usize,
    execute: fn(&mut Computer, &[Parameter]) -> Option<ComputerExitStatus>,
}

const OP_PAIRS: &[(u8, Operation)] = &[
    (
        1, // Add
        Operation {
            num_parameters: 3,
            execute: |computer, parameters| {
                let (operand1, operand2, dst) = unpack_params_3(parameters);
                let operand1 = computer.read_param(operand1);
                let operand2 = computer.read_param(operand2);
                computer.write_param(dst, operand1 + operand2);
                None
            },
        },
    ),
    (
        2, // Multiply
        Operation {
            num_parameters: 3,
            execute: |computer, parameters| {
                let (operand1, operand2, dst) = unpack_params_3(parameters);
                let operand1 = computer.read_param(operand1);
                let operand2 = computer.read_param(operand2);
                computer.write_param(dst, operand1 * operand2);
                None
            },
        },
    ),
    (
        3, // Save from input
        Operation {
            num_parameters: 1,
            execute: |computer, parameters| {
                let dst = unpack_params_1(parameters);
                if let Some(input) = computer.get_next_input() {
                    computer.write_param(dst, input);
                    None
                } else {
                    Some(ComputerExitStatus::WaitingForInput)
                }
            },
        },
    ),
    (
        4, // Write to output
        Operation {
            num_parameters: 1,
            execute: |computer, parameters| {
                let output = unpack_params_1(parameters);
                let output = computer.read_param(output);
                computer.write_to_output(output);
                None
            },
        },
    ),
    (
        5, // Jump if true
        Operation {
            num_parameters: 2,
            execute: |computer, parameters| {
                let (predicate, position) = unpack_params_2(parameters);
                let predicate = computer.read_param(predicate);
                let position = computer.read_param(position);
                if predicate != 0 {
                    computer.instruction_pointer = position as usize;
                }
                None
            },
        },
    ),
    (
        6, // Jump if false
        Operation {
            num_parameters: 2,
            execute: |computer, parameters| {
                let (predicate, position) = unpack_params_2(parameters);
                let predicate = computer.read_param(predicate);
                let position = computer.read_param(position);
                if predicate == 0 {
                    computer.instruction_pointer = position as usize;
                }
                None
            },
        },
    ),
    (
        7, // Less than
        Operation {
            num_parameters: 3,
            execute: |computer, parameters| {
                let (operand1, operand2, dst) = unpack_params_3(parameters);
                let operand1 = computer.read_param(operand1);
                let operand2 = computer.read_param(operand2);
                let value = i64::from(operand1 < operand2);
                computer.write_param(dst, value);
                None
            },
        },
    ),
    (
        8, // Equals
        Operation {
            num_parameters: 3,
            execute: |computer, parameters| {
                let (operand1, operand2, dst) = unpack_params_3(parameters);
                let operand1 = computer.read_param(operand1);
                let operand2 = computer.read_param(operand2);
                let value = i64::from(operand1 == operand2);
                computer.write_param(dst, value);
                None
            },
        },
    ),
    (
        9, // Adjust relative base
        Operation {
            num_parameters: 1,
            execute: |computer, parameters| {
                let offset = unpack_params_1(parameters);
                let offset = computer.read_param(offset);
                computer.relative_base += offset;
                None
            },
        },
    ),
    (
        99, // Halt
        Operation {
            num_parameters: 0,
            execute: |computer, parameters| {
                unpack_params_0(parameters);
                computer.terminated = true;
                Some(ComputerExitStatus::Finished)
            },
        },
    ),
];

lazy_static! {
    static ref OPS: HashMap<u8, Operation> = OP_PAIRS.iter().cloned().collect();
}

fn unpack_params_0(parameters: &[Parameter]) {
    assert_eq!(parameters.len(), 0);
}

fn unpack_params_1(parameters: &[Parameter]) -> &Parameter {
    assert_eq!(parameters.len(), 1);
    &parameters[0]
}

fn unpack_params_2(parameters: &[Parameter]) -> (&Parameter, &Parameter) {
    assert_eq!(parameters.len(), 2);
    (&parameters[0], &parameters[1])
}

fn unpack_params_3(parameters: &[Parameter]) -> (&Parameter, &Parameter, &Parameter) {
    assert_eq!(parameters.len(), 3);
    (&parameters[0], &parameters[1], &parameters[2])
}

#[must_use]
#[derive(Debug, Eq, PartialEq)]
pub enum ComputerExitStatus {
    Finished,
    WaitingForInput,
}

impl ComputerExitStatus {
    pub fn assert_finished(&self) {
        match self {
            ComputerExitStatus::Finished => {}
            _ => panic!("Expected status to be Finished, actually got: {:?}", self),
        }
    }
    pub fn assert_waiting_for_input(&self) {
        match self {
            ComputerExitStatus::WaitingForInput => {}
            _ => panic!(
                "Expected status to be WaitingForInput, actually got: {:?}",
                self
            ),
        }
    }
}

struct Memory {
    dense: Vec<Word>,
    sparse: HashMap<usize, Word>,
}

impl Memory {
    fn new(dense: Vec<Word>) -> Self {
        Memory {
            dense,
            sparse: Default::default(),
        }
    }

    fn read(&self, position: usize) -> Word {
        *self
            .dense
            .get(position)
            .unwrap_or_else(|| self.sparse.get(&position).unwrap_or(&0))
    }

    fn write(&mut self, position: usize, value: Word) {
        if position < self.dense.len() {
            self.dense[position] = value;
        } else {
            self.sparse.insert(position, value);
        }
    }
}

pub struct Computer {
    memory: Memory,
    relative_base: Word,
    instruction_pointer: usize,
    terminated: bool,
    inputs: VecDeque<Word>,
    outputs: Vec<Word>,
}

impl Computer {
    pub fn from_packed(packed: &str) -> Self {
        Self {
            memory: Memory::new(packed.split(',').map(|d| d.parse().expect(d)).collect()),
            relative_base: 0,
            instruction_pointer: 0,
            terminated: false,
            inputs: VecDeque::new(),
            outputs: Vec::new(),
        }
    }

    pub fn write_memory(&mut self, position: usize, value: Word) {
        self.memory.write(position, value);
    }

    fn decompose_op_code(mut raw_op_code: Word) -> (u8, Vec<ParameterMode>) {
        let op_code = (raw_op_code % 100) as u8;
        let mut modes = Vec::new();
        raw_op_code /= 100;
        while raw_op_code != 0 {
            modes.push(ParameterMode::from_i64(raw_op_code % 10).unwrap());
            raw_op_code /= 10;
        }
        (op_code, modes)
    }

    pub fn run(&mut self) -> ComputerExitStatus {
        assert!(!self.terminated, "Cannot run once finished");
        loop {
            let (op_code, parameter_modes) =
                Computer::decompose_op_code(self.memory.read(self.instruction_pointer));

            let operation = match OPS.get(&op_code) {
                Some(op) => op,
                None => panic!("Unknown opcode: {}", op_code),
            };

            let mut parameters = Vec::new();
            for param_idx in 0..operation.num_parameters {
                parameters.push(Parameter {
                    mode: parameter_modes
                        .get(param_idx)
                        .cloned()
                        .unwrap_or(ParameterMode::Position),
                    value: self.memory.read(self.instruction_pointer + 1 + param_idx),
                });
            }

            let orig_instruction_pointer = self.instruction_pointer;
            // eprintln!("op_code = {:?}, parameters = {:?}", op_code, parameters);
            // eprintln!("memory = {:?}", self.memory);
            let execution_result = (operation.execute)(self, parameters.as_slice());
            if let Some(status) = execution_result {
                return status;
            }

            if orig_instruction_pointer == self.instruction_pointer {
                // If we didn't change it, then increment it
                self.instruction_pointer += 1 + operation.num_parameters;
            }
        }
    }

    fn read_param(&self, parameter: &Parameter) -> Word {
        match parameter.mode {
            ParameterMode::Position => self.memory.read(parameter.value as usize),
            ParameterMode::Immediate => parameter.value,
            ParameterMode::Relative => self
                .memory
                .read((parameter.value + self.relative_base) as usize),
        }
    }

    fn write_param(&mut self, parameter: &Parameter, value: Word) {
        match parameter.mode {
            ParameterMode::Position => self.memory.write(parameter.value as usize, value),
            ParameterMode::Immediate => {
                panic!("Doesn't make sense to write to an immediate parameter!")
            }
            ParameterMode::Relative => self
                .memory
                .write((parameter.value + self.relative_base) as usize, value),
        }
    }

    pub fn send_as_input(&mut self, value: Word) {
        self.inputs.push_back(value);
    }

    fn get_next_input(&mut self) -> Option<Word> {
        self.inputs.pop_front()
    }

    fn write_to_output(&mut self, value: Word) {
        self.outputs.push(value);
    }

    pub fn get_value_at(&self, address: usize) -> Word {
        self.memory.read(address)
    }

    pub fn memory(&self) -> &[Word] {
        self.memory.dense.as_slice()
    }

    pub fn outputs(&self) -> &[Word] {
        self.outputs.as_slice()
    }

    pub fn drain_outputs(&mut self) -> Vec<Word> {
        let mut tmp_outputs = Vec::new();
        mem::swap(&mut tmp_outputs, &mut self.outputs);
        tmp_outputs
    }
}
