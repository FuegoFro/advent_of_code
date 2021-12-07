use crate::util::p_i32;
use itertools::Itertools;

fn get_cost(positions: &Vec<i32>, target: i32, cost_func: &impl Fn(i32, i32) -> i32) -> i32 {
    positions.iter().map(|p| cost_func(target, *p)).sum()
}

fn find_minimal_cost(positions: &Vec<i32>, cost_func: impl Fn(i32, i32) -> i32) -> i32 {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    (min..=max)
        .map(|target| get_cost(positions, target, &cost_func))
        .min()
        .unwrap()
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace("\r", "");

    let positions = input.split(",").map(p_i32).collect_vec();

    println!(
        "Part 1: {}",
        find_minimal_cost(&positions, |a, b| (a - b).abs())
    );
    println!(
        "Part 2: {}",
        find_minimal_cost(&positions, |a, b| {
            let diff = (a - b).abs();
            diff * (diff + 1) / 2
        })
    );
}
