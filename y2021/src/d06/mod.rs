use itertools::Itertools;
use std::collections::HashMap;
use util::p_u32;

fn simulate(mut fish_counts: HashMap<u32, u64>, days: usize) -> u64 {
    for _ in 0..days {
        let mut next_fish_counts = HashMap::new();
        for (mut age, number) in fish_counts {
            if age == 0 {
                next_fish_counts.insert(8, number);
                age = 7;
            }
            *next_fish_counts.entry(age - 1).or_default() += number;
        }
        fish_counts = next_fish_counts;
    }
    fish_counts.values().sum()
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace("\r", "");

    let fish = input.split(",").map(p_u32).collect_vec();
    let mut fish_counts = HashMap::new();
    for age in fish {
        *fish_counts.entry(age).or_insert(0u64) += 1;
    }

    println!("Part 1: {}", simulate(fish_counts.clone(), 80));
    println!("Part 2: {}", simulate(fish_counts.clone(), 256));
}
