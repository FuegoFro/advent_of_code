use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use tuple::Map;

fn signal_str_to_set(s: &&str) -> HashSet<char> {
    s.chars().collect()
}

/// Brute force decoding things.
///
/// We know we're going to see exactly one of each digit in the signals, so we can use the
/// following ~~heuristics~~ *a l g o r i t h m* to determine which set of letters corresponds
/// to which digit. The following is a table for going from <number of letters in signal> to digit:
///
/// 2 -> 1
/// 4 -> 4
/// 3 -> 7
/// 7 -> 8
/// 6 (depends on how they compare to previous)
///     Overlap w/ 4 -> 9
///     Else overlap w/ 1 -> 0 (can determine center center wire)
///     Else -> 6
/// 5 (depends on how they compare to previous)
///     Overlap w/ 7 -> 3 (can determine top center wire)
///     Overlaps 9 -> 5
///     Else 2
fn decode_entry((signals, queries): &(Vec<&str>, Vec<&str>)) -> u32 {
    // Group signals by length
    let signals_by_length: HashMap<usize, Vec<HashSet<char>>> = signals
        .iter()
        .map(|s| (s.len(), signal_str_to_set(s)))
        .into_group_map();

    // Run the heuristics to determine signal <-> digit mapping
    let mut signals_by_digit: HashMap<u32, &HashSet<char>> = HashMap::from([
        (1, &signals_by_length[&2][0]),
        (4, &signals_by_length[&4][0]),
        (7, &signals_by_length[&3][0]),
        (8, &signals_by_length[&7][0]),
    ]);
    for len_6 in &signals_by_length[&6] {
        if signals_by_digit[&4].is_subset(&len_6) {
            signals_by_digit.insert(9, &len_6);
        } else if signals_by_digit[&1].is_subset(&len_6) {
            signals_by_digit.insert(0, &len_6);
        } else {
            signals_by_digit.insert(6, &len_6);
        }
    }
    // Note relies on entry for 9 existing, which is done in the len_6 block above.
    for len_5 in &signals_by_length[&5] {
        if signals_by_digit[&7].is_subset(&len_5) {
            signals_by_digit.insert(3, &len_5);
        } else if len_5.is_subset(&signals_by_digit[&9]) {
            signals_by_digit.insert(5, &len_5);
        } else {
            signals_by_digit.insert(2, &len_5);
        }
    }

    // Finally translate the query
    queries
        .iter()
        .map(signal_str_to_set)
        .map(|query| {
            signals_by_digit
                .iter()
                .filter_map(|(digit, signal)| if **signal == query { Some(digit) } else { None })
                .next()
                .unwrap()
        })
        .rev()
        .enumerate()
        .map(|(idx, digit)| digit * 10u32.pow(idx as u32 + 1))
        .sum()
}

pub fn main() {
    let input = include_str!("example_input.txt").trim().replace("\r", "");
    // let input = include_str!("actual_input.txt").trim().replace("\r", "");

    let raw_entries = input
        .split("\n")
        .map(|entry| {
            entry
                .split_once(" | ")
                .unwrap()
                .map(|part| part.split(" ").collect_vec())
        })
        .collect_vec();

    let num_unique_digits = raw_entries
        .iter()
        .flat_map(|(_, entry)| {
            entry
                .iter()
                .filter(|d| [2usize, 4, 3, 7].contains(&d.len()))
        })
        .count();
    println!("Part 1: {}", num_unique_digits);

    let all_queries: u32 = raw_entries.iter().map(decode_entry).sum();
    println!("Part 2: {}", all_queries);
}
