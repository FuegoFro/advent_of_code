mod pt1 {
    use regex::Regex;
    use std::collections::HashMap;
    use util::p_u64;

    enum Instruction {
        SetMask { and_mask: u64, or_mask: u64 },
        SetMemory { address: u64, value: u64 },
    }

    impl Instruction {
        fn from_packed(packed: &str) -> Self {
            lazy_static! {
                static ref RE_MASK: Regex = Regex::new(r"^mask = (?P<mask>[X01]{36})$").unwrap();
                static ref RE_MEM: Regex =
                    Regex::new(r"^mem\[(?P<addr>\d+)\] = (?P<val>\d+)$").unwrap();
            }
            // IntelliJ doesn't understand this without this alias.
            let re_mask: &Regex = &RE_MASK;
            let re_mem: &Regex = &RE_MEM;
            if let Some(mask_caps) = re_mask.captures(packed) {
                let mask_raw = mask_caps.name("mask").unwrap().as_str();
                Instruction::SetMask {
                    and_mask: u64::from_str_radix(&mask_raw.replace("X", "1"), 2).unwrap(),
                    or_mask: u64::from_str_radix(&mask_raw.replace("X", "0"), 2).unwrap(),
                }
            } else if let Some(mem_caps) = re_mem.captures(packed) {
                Instruction::SetMemory {
                    address: p_u64(mem_caps.name("addr").unwrap().as_str()),
                    value: p_u64(mem_caps.name("val").unwrap().as_str()),
                }
            } else {
                panic!("Unable to parse instruction: {}", packed);
            }
        }
    }

    #[derive(Default)]
    struct Program {
        and_mask: u64,
        or_mask: u64,
        mem: HashMap<u64, u64>,
    }

    impl Program {
        fn execute_instruction(&mut self, instruction: Instruction) {
            match instruction {
                Instruction::SetMask { and_mask, or_mask } => {
                    self.and_mask = and_mask;
                    self.or_mask = or_mask;
                }
                Instruction::SetMemory { address, value } => {
                    self.mem
                        .insert(address, (value & self.and_mask) | self.or_mask);
                }
            }
        }

        fn sum_memory(&self) -> u64 {
            self.mem.values().sum()
        }
    }

    pub fn do_pt1(input: &str) {
        let mut program = Program::default();
        input
            .split("\n")
            .map(Instruction::from_packed)
            .for_each(|i| program.execute_instruction(i));
        println!("{}", program.sum_memory());
    }
}

mod pt2 {
    use regex::Regex;
    use std::collections::HashMap;
    use util::p_u64;

    #[derive(Debug)]
    enum Instruction {
        SetMask {
            base_mask: u64,
            floating_bits: Vec<u64>,
        },
        SetMemory {
            address: u64,
            value: u64,
        },
    }

    impl Instruction {
        fn from_packed(packed: &str) -> Self {
            lazy_static! {
                static ref RE_MASK: Regex = Regex::new(r"^mask = (?P<mask>[X01]{36})$").unwrap();
                static ref RE_MEM: Regex =
                    Regex::new(r"^mem\[(?P<addr>\d+)\] = (?P<val>\d+)$").unwrap();
            }
            // IntelliJ doesn't understand this without this alias.
            let re_mask: &Regex = &RE_MASK;
            let re_mem: &Regex = &RE_MEM;
            if let Some(mask_caps) = re_mask.captures(packed) {
                let mask_raw = mask_caps.name("mask").unwrap().as_str();
                Instruction::SetMask {
                    base_mask: u64::from_str_radix(&mask_raw.replace("X", "0"), 2).unwrap(),
                    floating_bits: mask_raw
                        .chars()
                        .enumerate()
                        .filter(|(_, c)| *c == 'X')
                        .map(|(i, _)| (mask_raw.len() - 1 - i) as u64)
                        .collect(),
                }
            } else if let Some(mem_caps) = re_mem.captures(packed) {
                Instruction::SetMemory {
                    address: p_u64(mem_caps.name("addr").unwrap().as_str()),
                    value: p_u64(mem_caps.name("val").unwrap().as_str()),
                }
            } else {
                panic!("Unable to parse instruction: {}", packed);
            }
        }
    }

    #[derive(Default, Debug)]
    struct Program {
        base_mask: u64,
        floating_bits: Vec<u64>,
        mem: HashMap<u64, u64>,
    }

    impl Program {
        fn execute_instruction(&mut self, instruction: Instruction) {
            match instruction {
                Instruction::SetMask {
                    base_mask,
                    floating_bits,
                } => {
                    self.base_mask = base_mask;
                    self.floating_bits = floating_bits;
                }
                Instruction::SetMemory { address, value } => {
                    let base_address = address | self.base_mask;
                    for mut i in 0..(2 as u32).pow(self.floating_bits.len() as u32) {
                        let mut current_address = base_address;
                        for bit_idx in self.floating_bits.iter() {
                            let is_one = i & 1 == 1;
                            i >>= 1;
                            let mask = 1 << bit_idx;
                            if is_one {
                                current_address |= mask;
                            } else {
                                current_address &= !mask;
                            }
                        }

                        self.mem.insert(current_address, value);
                    }
                }
            }
        }

        fn sum_memory(&self) -> u64 {
            self.mem.values().sum()
        }
    }

    pub fn do_pt2(input: &str) {
        let mut program = Program::default();
        input
            .split("\n")
            .map(Instruction::from_packed)
            .for_each(|i| program.execute_instruction(i));
        println!("{}", program.sum_memory());
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    pt1::do_pt1(input);
    pt2::do_pt2(input);
}
