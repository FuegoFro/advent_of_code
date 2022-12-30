use itertools::Itertools;
use util::p_u32;

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let sorted_calories_per_elf = input
        .split("\n\n")
        .map(|l| l.split('\n').map(p_u32).sum())
        .sorted()
        .rev()
        .collect::<Vec<u32>>();

    println!("Part 1: {}", sorted_calories_per_elf[0]);
    println!(
        "Part 2: {}",
        sorted_calories_per_elf.iter().take(3).sum::<u32>()
    );
}
