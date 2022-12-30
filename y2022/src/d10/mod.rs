use itertools::{join, Itertools};
use num_traits::{abs, abs_sub};
use util::p_i32;

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        if s == "noop" {
            Instruction::Noop
        } else if let Some(val) = s.strip_prefix("addx ") {
            Instruction::AddX(p_i32(val))
        } else {
            panic!("Unknown instruction str {:?}", s);
        }
    }
}

fn do_mid_cycle(cycle: i32, register: i32, signal_strengths: &mut i32, pixels: &mut [bool]) {
    let col = cycle.rem_euclid(40);
    if col == 19 {
        *signal_strengths += register * (cycle + 1);
    }
    // println!(
    //     "During cycle  {}: CRT draws pixel in position {}",
    //     cycle + 1,
    //     col
    // );
    pixels[cycle as usize] = (register - col).abs() <= 1;
}

fn render_pixels(pixels: &[bool]) -> String {
    pixels
        .iter()
        .map(|p| if *p { '#' } else { ' ' })
        .chunks(40)
        .into_iter()
        .map(|c| c.into_iter().join(""))
        .join("\n")
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let instructions = input.split('\n').map(Instruction::from_str).collect_vec();

    let mut pixels = [false; 240];
    let mut signal_strengths = 0;
    let mut register = 1;
    let mut cycle = 0;
    for instruction in instructions.iter() {
        // println!(
        //     "Start cycle   {}: begin executing {:?}",
        //     cycle + 1,
        //     &instruction
        // );
        match instruction {
            Instruction::Noop => {
                do_mid_cycle(cycle, register, &mut signal_strengths, &mut pixels);
                cycle += 1;
            }
            Instruction::AddX(val) => {
                do_mid_cycle(cycle, register, &mut signal_strengths, &mut pixels);
                cycle += 1;
                do_mid_cycle(cycle, register, &mut signal_strengths, &mut pixels);
                register += val;
                cycle += 1;
            }
        }
        // println!(
        //     "End of cycle  {}: finishing executing {:?} (Register X is now {})",
        //     cycle + 1,
        //     &instruction,
        //     register,
        // );
        // let sprite_pos = (0..40)
        //     // .map(|p| if abs_sub(p, register) <= 1 { '#' } else { '.' })
        //     .map(|p| format!("{} ", (p - register).abs()))
        //     .join("");
        // println!("Sprite position: {}", sprite_pos);
    }

    println!("Part 1: {}", signal_strengths);

    let image = render_pixels(&pixels);

    println!("Part 2:\n{}", image);
}
