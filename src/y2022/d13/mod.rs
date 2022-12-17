use itertools::EitherOrBoth::Both;
use itertools::{EitherOrBoth, Itertools};
use serde_json::Value;
use std::cmp::Ordering;
use tuple::Map;

#[derive(Eq, PartialEq)]
enum CompareOutcome {
    Correct,
    Incorrect,
    Undetermined,
}

fn compare(left: &Value, right: &Value) -> CompareOutcome {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => {
            if l == r {
                CompareOutcome::Undetermined
            } else if l.as_i64() < r.as_i64() {
                CompareOutcome::Correct
            } else {
                CompareOutcome::Incorrect
            }
        }
        (Value::Array(l), Value::Array(r)) => {
            for pair in l.iter().zip_longest(r.iter()) {
                match pair {
                    Both(l, r) => {
                        let comparison = compare(l, r);
                        if comparison != CompareOutcome::Undetermined {
                            return comparison;
                        }
                    }
                    EitherOrBoth::Left(_) => return CompareOutcome::Incorrect,
                    EitherOrBoth::Right(_) => return CompareOutcome::Correct,
                }
            }
            CompareOutcome::Undetermined
        }
        (l @ Value::Array(_), r @ Value::Number(_)) => compare(l, &Value::Array(vec![r.clone()])),
        (l @ Value::Number(_), r @ Value::Array(_)) => compare(&Value::Array(vec![l.clone()]), r),
        _ => panic!("Invalid combo {:?}", (left, right)),
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let packet_pairs = input
        .split("\n\n")
        .map(|packets| {
            packets
                .split_once('\n')
                .unwrap()
                .map(|p| p.parse::<Value>().unwrap())
        })
        .collect_vec();

    let summed_in_order_indices: usize = packet_pairs
        .iter()
        .enumerate()
        .map(|(i, (l, r))| {
            if compare(l, r) == CompareOutcome::Correct {
                i + 1
            } else {
                0
            }
        })
        .sum();

    println!("Part 1: {}", summed_in_order_indices);

    let mut all_packets = packet_pairs
        .into_iter()
        .flat_map(|(l, r)| [l, r].into_iter())
        .collect_vec();
    let key1 = "[[2]]".parse::<Value>().unwrap();
    let key2 = "[[6]]".parse::<Value>().unwrap();
    all_packets.push(key1.clone());
    all_packets.push(key2.clone());

    all_packets.sort_by(|l, r| match compare(l, r) {
        CompareOutcome::Correct => Ordering::Less,
        CompareOutcome::Incorrect => Ordering::Greater,
        CompareOutcome::Undetermined => Ordering::Equal,
    });

    let idx1 = all_packets.iter().position(|e| e == &key1).unwrap() + 1;
    let idx2 = all_packets.iter().position(|e| e == &key2).unwrap() + 1;

    println!("Part 2: {}", idx1 * idx2);
}
