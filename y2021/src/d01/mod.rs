use itertools::Itertools;
use util::p_u32;

fn get_num_increasing(nums: &Vec<u32>) -> u32 {
    nums.iter()
        .tuple_windows()
        .map(|(before, after)| if after > before { 1 } else { 0 })
        .sum()
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let nums = input.split("\n").map(p_u32).collect::<Vec<_>>();

    println!("Part 1: {}", get_num_increasing(&nums));

    let windows = nums
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .collect();
    println!("Part 2: {}", get_num_increasing(&windows));
}
