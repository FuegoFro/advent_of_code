use itertools::Itertools;
use std::collections::{HashSet};

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        _ => panic!("Invalid char {}", c),
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let bags = input
        .split('\n')
        .map(|l| {
            let half_way = l.len() / 2;
            let (first, second) = l.split_at(half_way);
            (
                HashSet::<_>::from_iter(first.chars()),
                HashSet::<_>::from_iter(second.chars()),
            )
        })
        .collect_vec();

    let total: u32 = bags
        .iter()
        .map(|(a, b)| {
            let common_keys = a.intersection(b).collect_vec();
            assert_eq!(common_keys.len(), 1);
            priority(*common_keys[0])
        })
        .sum();

    println!("Part 1: {}", total);

    let total: u32 = bags
        .iter()
        .chunks(3)
        .into_iter()
        .map(|c| {
            let common_keys = c
                .into_iter()
                .map(|(a, b)| a | b)
                .reduce(|accum, item| &accum & &item)
                .unwrap()
                .into_iter()
                .collect_vec();
            assert_eq!(common_keys.len(), 1);
            priority(common_keys[0])
        })
        .sum();

    println!("Part 2: {}", total);
}
