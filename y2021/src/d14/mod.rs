use std::collections::HashMap;

use itertools::{Itertools, MinMaxResult};
use util::iter_helpers::IteratorHelpers;

fn do_insertions(
    pair_counts: HashMap<(char, char), usize>,
    insertions: &HashMap<(char, char), char>,
) -> HashMap<(char, char), usize> {
    let mut new_pair_counts = HashMap::new();
    for ((a, b), count) in pair_counts {
        match insertions.get(&(a, b)).cloned() {
            Some(middle) => {
                *new_pair_counts.entry((a, middle)).or_default() += count;
                *new_pair_counts.entry((middle, b)).or_default() += count;
            }
            None => {
                *new_pair_counts.entry((a, b)).or_default() += count;
            }
        }
    }
    new_pair_counts
}

fn get_min_max_diff_after_cycles(
    mut pair_counts: HashMap<(char, char), usize>,
    insertions: &HashMap<(char, char), char>,
    last_char: char,
    num_cycles: usize,
) -> usize {
    for _ in 0..num_cycles {
        pair_counts = do_insertions(pair_counts, &insertions);
    }

    let mut counts = pair_counts
        .into_iter()
        // Ignore the second value since it'll be represented by the first part of the other pairs.
        // This holds true for every value in the string *except* the last one, hence the adjustment
        // for the last char.
        .map(|((a, _), count)| (a, count))
        .into_sum_map();
    *counts.entry(last_char).or_default() += 1;
    match counts.values().minmax() {
        MinMaxResult::MinMax(min, max) => max - min,
        _ => panic!(),
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace("\r", "");

    let (raw_start, raw_insertions) = input.split_once("\n\n").unwrap();

    let initial_polymer = raw_start.chars().collect_vec();
    let last_char = *initial_polymer.last().unwrap();

    let initial_pair_counts = initial_polymer
        .iter()
        .zip(initial_polymer.iter().skip(1))
        .map(|(a, b)| (*a, *b))
        .into_count_map();

    let insertions = raw_insertions
        .split("\n")
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(start_pair_raw, end)| {
            let pair = start_pair_raw
                .chars()
                .collect_tuple::<(char, char)>()
                .unwrap();
            let to_insert = end.chars().next().unwrap();
            (pair, to_insert)
        })
        .collect::<HashMap<_, _>>();

    let part1 =
        get_min_max_diff_after_cycles(initial_pair_counts.clone(), &insertions, last_char, 10);
    println!("Part 1: {}", part1);

    let part2 =
        get_min_max_diff_after_cycles(initial_pair_counts.clone(), &insertions, last_char, 40);
    println!("Part 2: {}", part2);
}
