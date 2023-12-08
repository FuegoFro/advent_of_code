use itertools::Itertools;
use recap::Recap;
use serde::Deserialize;
use std::collections::HashMap;

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
        .fold(
            (1, 0),
            |(existing_period, existing_phase), (new_offset, new_period)| {
                combine_phased_rotations(existing_period, existing_phase, new_period, -new_offset)
                    .unwrap()
            },
        );

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

// Taken from https://math.stackexchange.com/a/3864593/1264446
use std::error::Error;

/// Extended Greatest Common Divisor Algorithm
///
/// Returns:
///     gcd: The greatest common divisor of a and b.
///     s, t: Coefficients such that s*a + t*b = gcd
///
/// Reference:
///     https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Pseudocode
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let quotient = old_r / r;
        let remainder = old_r % r;
        old_r = r;
        r = remainder;
        let new_s = old_s - quotient * s;
        let new_t = old_t - quotient * t;
        old_s = std::mem::replace(&mut s, new_s);
        old_t = std::mem::replace(&mut t, new_t);
    }

    (old_r, old_s, old_t)
}

/// Combine two phased rotations into a single phased rotation
///
/// Returns: combined_period, combined_phase
///
/// The combined rotation is at its reference point if and only if both a and b
/// are at their reference points.
fn combine_phased_rotations(
    a_period: i64,
    a_phase: i64,
    b_period: i64,
    b_phase: i64,
) -> Result<(i64, i64), Box<dyn Error>> {
    let (gcd, s, _t) = extended_gcd(a_period, b_period);
    let phase_difference = a_phase - b_phase;
    let (pd_mult, pd_remainder) = (phase_difference / gcd, phase_difference % gcd);

    if pd_remainder != 0 {
        return Err("Rotation reference points never synchronize.".into());
    }

    let combined_period = a_period / gcd * b_period;
    let combined_phase = (a_phase - s * pd_mult * a_period) % combined_period;
    Ok((combined_period, combined_phase))
}
