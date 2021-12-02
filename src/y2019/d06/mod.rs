use crate::util::split_once;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

const EMPTY_VEC: Vec<&str> = Vec::new();

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let graph = input.lines().map(|l| split_once(l, ")")).into_group_map();
    let reverse_graph = graph
        .iter()
        .flat_map(|(k, v)| v.iter().map(move |val| (*val, *k)))
        .into_group_map();

    let mut frontier = VecDeque::new();
    frontier.push_back((0, "COM"));
    let mut total = 0;
    while let Some((depth, node)) = frontier.pop_front() {
        total += depth;
        for orbiter in graph.get(node).unwrap_or(&EMPTY_VEC).iter() {
            frontier.push_back((depth + 1, orbiter));
        }
    }

    println!("{}", total);

    let mut frontier = VecDeque::new();
    let mut seen = HashSet::new();
    frontier.push_back((0, "YOU"));
    while let Some((depth, node)) = frontier.pop_front() {
        if !seen.insert(node) {
            continue;
        }
        if node == "SAN" {
            println!("{}", depth - 2);
        }
        for orbiter in graph
            .get(node)
            .unwrap_or(&EMPTY_VEC)
            .iter()
            .chain(reverse_graph.get(node).unwrap_or(&EMPTY_VEC).iter())
        {
            frontier.push_back((depth + 1, orbiter));
        }
    }
}
