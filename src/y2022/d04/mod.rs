use crate::util::p_u32;
use itertools::Itertools;
use tuple::Map;

type Range = (u32, u32);

fn contains(a: &Range, b: &Range) -> bool {
    a.0 <= b.0 && b.1 <= a.1
}

fn any_overlap(a: &Range, b: &Range) -> bool {
    a.0 <= b.0 && b.0 <= a.1 // supposed to swap vars? || a.0 <= b.0 && b.0 <= a.1
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let ranges = input
        .split('\n')
        .map(|l| {
            l.split_once(',')
                .unwrap()
                .map(|s| s.split_once('-').unwrap().map(p_u32))
        })
        .collect_vec();

    let num_fully_contained = ranges
        .iter()
        .filter(|(a, b)| contains(a, b) || contains(b, a))
        .count();

    println!("Part 1: {}", num_fully_contained);

    let num_any_overlap = ranges
        .iter()
        .filter(|(a, b)| any_overlap(a, b) || any_overlap(b, a))
        .count();

    println!("Part 2: {}", num_any_overlap);
}
