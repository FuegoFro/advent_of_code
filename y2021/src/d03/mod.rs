use itertools::Itertools;
use std::ops::Not;
use util::p_u32c;

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let lines = input
        .split_whitespace()
        .map(|line| line.chars().map(p_u32c).collect_vec())
        .collect_vec();
    let half_lines = (lines.len() / 2) as u32;

    let element_wise_summed = lines
        .clone()
        .into_iter()
        .reduce(|a, b| a.iter().zip(b.iter()).map(|(a, b)| a + b).collect_vec())
        .unwrap();

    let majority_value = element_wise_summed.iter().fold(0u32, |accum, bit_count| {
        let bit_value = if *bit_count > half_lines { 1 } else { 0 };
        (accum << 1) | bit_value
    });
    let mask = (0u32.not() << element_wise_summed.len()).not();
    let minority_value = majority_value.not() & mask;

    println!("Part 1: {}", majority_value * minority_value);

    let majority_value = narrow_down_numbers(&lines, |num_ones, half| num_ones >= half);
    let minority_value = narrow_down_numbers(&lines, |num_ones, half| num_ones < half);
    println!("Part 2: {}", majority_value * minority_value);
}

fn narrow_down_numbers(numbers: &Vec<Vec<u32>>, pick_bit: impl Fn(f32, f32) -> bool) -> u32 {
    let mut numbers = numbers.clone();
    for bit_position in 0..numbers[0].len() {
        if numbers.len() == 1 {
            break;
        }
        let num_ones: f32 = numbers.iter().map(|n| n[bit_position] as f32).sum();
        let half = numbers.len() as f32 / 2f32;
        let expected_bit = if pick_bit(num_ones, half) { 1 } else { 0 };
        numbers = numbers
            .into_iter()
            .filter(|n| n[bit_position] == expected_bit)
            .collect_vec();
    }
    numbers[0]
        .iter()
        .fold(0u32, |accum, bit| (accum << 1) | bit)
}
