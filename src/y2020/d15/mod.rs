use crate::util::p_u32;
use std::collections::HashMap;

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let mut numbers = input.split(",").map(p_u32).collect::<Vec<_>>();
    let mut most_recent = numbers[0..(numbers.len() - 1)]
        .iter()
        .enumerate()
        .map(|(i, n)| (*n, i))
        .collect::<HashMap<_, _>>();

    for i in (numbers.len() - 1)..(30000000 - 1) {
        let last_num = *numbers.last().unwrap();
        let age = most_recent
            .get(&last_num)
            .map(|r| (i - r) as u32)
            .unwrap_or(0);
        numbers.push(age);
        most_recent.insert(last_num, i);
    }
    println!("{}", numbers.last().unwrap());
}
