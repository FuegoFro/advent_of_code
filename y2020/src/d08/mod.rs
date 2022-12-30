use std::collections::HashSet;

#[derive(Debug)]
enum Operation {
    Accumulate,
    Jump,
    NoOp,
}

impl Operation {
    fn from_packed(packed: &str) -> Self {
        match packed {
            "acc" => Operation::Accumulate,
            "jmp" => Operation::Jump,
            "nop" => Operation::NoOp,
            _ => panic!("Unknown opcode {}", packed),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    op: Operation,
    arg: i32,
}

impl Instruction {
    fn from_packed(packed: &str) -> Self {
        let mut parts = packed.split(' ');
        Instruction {
            op: Operation::from_packed(parts.next().unwrap()),
            arg: parts.next().unwrap().parse::<i32>().unwrap(),
        }
    }
}

struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn from_packed(packed: &str) -> Self {
        Program {
            instructions: packed
                .split('\n')
                .map(Instruction::from_packed)
                .collect(),
        }
    }

    fn accumulator_value_at_loop_or_finish(&self) -> Result<i32, i32> {
        let mut accumulator = 0;
        let mut pc = 0;
        let mut seen_pcs = HashSet::new();
        while let Some(instruction) = self.instructions.get(pc as usize) {
            if seen_pcs.contains(&pc) {
                break;
            }
            seen_pcs.insert(pc);
            match instruction.op {
                Operation::Accumulate => accumulator += instruction.arg,
                Operation::Jump => pc += instruction.arg,
                Operation::NoOp => {}
            }
            match instruction.op {
                // We've already set the pc to the right place for jumps
                Operation::Jump => {}
                _ => pc += 1,
            }
        }

        if pc as usize >= self.instructions.len() {
            Ok(accumulator)
        } else {
            Err(accumulator)
        }
    }

    fn corrected_output(&mut self) -> i32 {
        for instruction_idx_to_change in 0..self.instructions.len() {
            let (actual, temp) = match self.instructions[instruction_idx_to_change].op {
                Operation::Accumulate => continue,
                Operation::Jump => (Operation::Jump, Operation::NoOp),
                Operation::NoOp => (Operation::NoOp, Operation::Jump),
            };
            self.instructions[instruction_idx_to_change].op = temp;
            if let Ok(answer) = self.accumulator_value_at_loop_or_finish() {
                return answer;
            }
            self.instructions[instruction_idx_to_change].op = actual;
        }
        panic!("Could not find correct instruction")
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let mut program = Program::from_packed(input);
    let pt1 = program.accumulator_value_at_loop_or_finish().unwrap_err();
    println!("pt 1: {}", pt1);

    let pt2 = program.corrected_output();
    println!("pt 2: {}", pt2);
}
