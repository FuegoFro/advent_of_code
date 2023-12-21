use std::collections::HashMap;

use itertools::Itertools;
use recap::Recap;
use serde::Deserialize;

use util::cycle_helpers::FirstCommonCycle;

enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Unknown direction char {}", c),
        }
    }
}

#[derive(Deserialize, Recap)]
#[recap(regex = r"(?P<name>\w+) = \((?P<left>\w+), (?P<right>\w+)\)")]
struct NodeRaw {
    name: String,
    left: String,
    right: String,
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let (instructions_raw, graph_raw) = input.split_once("\n\n").unwrap();
    let instructions = instructions_raw
        .chars()
        .map(Direction::from_char)
        .collect_vec();
    let graph = graph_raw
        .lines()
        .map(|l| l.parse::<NodeRaw>().unwrap())
        .fold(HashMap::new(), |mut graph, node| {
            graph.insert(node.name, (node.left, node.right));
            graph
        });

    // let steps = get_num_steps(&instructions, &graph);
    //
    // // println!("Part 1: {}", steps);
    //
    // let mut current = graph
    //     .keys()
    //     .filter(|k| k.ends_with('A'))
    //     .map(|k| k.as_str())
    //     .collect_vec();
    // let mut steps = 0;
    // let mut instructions_iter = instructions.iter().cycle();
    // while !current.iter().all(|c| c.ends_with('Z')) {
    //     let instruction = instructions_iter.next().unwrap();
    //     current.iter_mut().for_each(|c| {
    //         let (left, right) = &graph[*c];
    //         *c = match instruction {
    //             Direction::Left => left.as_str(),
    //             Direction::Right => right.as_str(),
    //         };
    //     });
    //     steps += 1;
    // }

    let cycle_lengths = graph
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| get_num_steps(k, &instructions, &graph))
        // Figure out the first time these intersect
        .find_first_common_cycle();

    println!("Part 2: {:?}", cycle_lengths);
}

fn get_num_steps(
    start: &str,
    instructions: &[Direction],
    graph: &HashMap<String, (String, String)>,
) -> (i64, i64) {
    let mut current = start;
    let mut steps = 0;
    let mut first_seen = HashMap::new();
    let mut instructions_iter = instructions.iter().cycle();
    while !first_seen.contains_key(current) || !current.ends_with('Z') {
        first_seen.insert(current, steps);
        let (left, right) = &graph[current];
        current = match instructions_iter.next().unwrap() {
            Direction::Left => left.as_str(),
            Direction::Right => right.as_str(),
        };
        steps += 1;
    }
    let cycle_first = first_seen[current];
    (cycle_first, steps - cycle_first)
}
