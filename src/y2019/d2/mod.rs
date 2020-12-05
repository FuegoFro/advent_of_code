use anyhow::Result;

struct Program {
    data: Vec<usize>,
    pc: usize,
    is_running: bool,
}

impl Program {
    fn from_packed(packed: &str) -> Result<Self> {
        Ok(Self {
            data: packed
                .split(",")
                .map(|d| d.parse())
                .collect::<Result<_, _>>()?,
            pc: 0,
            is_running: true,
        })
    }

    fn fixup(&mut self, noun: usize, verb: usize) {
        self.data[1] = noun;
        self.data[2] = verb;
    }

    fn run(&mut self) {
        while self.is_running {
            match self.data[self.pc] {
                opcode @ 1 | opcode @ 2 => {
                    let src1 = self.data[self.data[self.pc + 1]];
                    let src2 = self.data[self.data[self.pc + 2]];
                    let dst_idx = self.data[self.pc + 3];
                    let value = if opcode == 1 {
                        src1 + src2
                    } else {
                        src1 * src2
                    };
                    self.data[dst_idx] = value;
                }
                99 => self.is_running = false,
                opcode => panic!("Unknown opcode {}", opcode),
            }
            self.pc += 4;
        }
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    // for line in input.split("\n") {
    //     let mut program = Program::from_packed(line).unwrap();
    //     program.run();
    //     println!("{:?}", program.data);
    // }

    let input = include_str!("actual_input.txt").trim();

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = Program::from_packed(input).unwrap();
            program.fixup(noun, verb);
            program.run();
            if program.data[0] == 19690720 {
                println!("{}", noun * 100 + verb);
                return;
            }
        }
    }
}
