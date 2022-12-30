#![allow(dead_code)]
// # Operations needed
// ## Explode
//  - Modify left/right (traversing upward)
//      - Maybe done by backtracking recursion?
//  - Replace self
// ## Split
//  - Replace self

use itertools::Itertools;
use num::Integer;
use std::cmp::max;
use std::fmt::{Debug, Formatter};
use util::p_u32c;

#[derive(Clone, Eq, PartialEq)]
enum NodeValue {
    SubNode(Box<Node>),
    Value(u32),
}

impl Debug for NodeValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeValue::SubNode(sub_node) => f.write_fmt(format_args!("{:?}", sub_node)),
            NodeValue::Value(v) => f.write_fmt(format_args!("{}", v)),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Node {
    left: NodeValue,
    right: NodeValue,
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{:?},{:?}]", self.left, self.right))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn get_value_mut<'a>(&self, node: &'a mut Node) -> &'a mut NodeValue {
        match self {
            Direction::Left => &mut node.left,
            Direction::Right => &mut node.right,
        }
    }
    fn get_opposite_value_mut<'a>(&self, node: &'a mut Node) -> &'a mut NodeValue {
        match self {
            Direction::Left => &mut node.right,
            Direction::Right => &mut node.left,
        }
    }
}

#[derive(Debug)]
enum ExplodeResult {
    ExplodeParent {
        replacement: u32,
        left: u32,
        right: u32,
    },
    ExplodeTraverse {
        value: u32,
        direction: Direction,
    },
    DidWork,
    None,
}

impl ExplodeResult {
    fn did_work(&self) -> bool {
        !matches!(self, Self::None)
    }
}

fn parse(s: &str) -> Node {
    // println!("Parsing {}", s);
    match parse_inner(s).1 {
        NodeValue::SubNode(n) => *n,
        NodeValue::Value(_) => panic!("Unable to parse `{}`", s),
    }
}

fn parse_inner(s: &str) -> (usize, NodeValue) {
    // println!("Parse inner called with {}", s);
    let first_char = s.chars().next().unwrap();
    let (first_consumed, left) = match first_char {
        '[' => parse_inner(&s[1..]),
        '0'..='9' => return (1, NodeValue::Value(p_u32c(first_char))),
        _ => panic!("Unknown char {}", first_char),
    };
    let s = &s[1 + first_consumed..];
    // println!("Parse inner first took {}, now has {}", first_consumed, s);

    let middle_char = s.chars().next().unwrap();
    assert_eq!(middle_char, ',');
    let s = &s[1..];
    // println!("Parse inner after comma has {}", s);

    let (second_consumed, right) = parse_inner(s);
    let s = &s[second_consumed..];

    let last_char = s.chars().next().unwrap();
    assert_eq!(last_char, ']');

    (
        // 3 for '[', ',', and ']'
        3 + first_consumed + second_consumed,
        NodeValue::SubNode(Box::new(Node { left, right })),
    )
}

fn explode(node: &mut Node, level: usize) -> ExplodeResult {
    if level == 4 {
        // Explode
        let left = match &node.left {
            NodeValue::SubNode(_) => panic!("Nested more than 4 level!"),
            NodeValue::Value(v) => v,
        };
        let right = match &node.right {
            NodeValue::SubNode(_) => panic!("Nested more than 4 level!"),
            NodeValue::Value(v) => v,
        };
        return ExplodeResult::ExplodeParent {
            replacement: 0,
            left: *left,
            right: *right,
        };
    }

    let left_result = explode_side(node, level, Direction::Left);
    if left_result.did_work() {
        return left_result;
    }

    explode_side(node, level, Direction::Right)
}

fn explode_side(node: &mut Node, level: usize, side_direction: Direction) -> ExplodeResult {
    let result = match side_direction.get_value_mut(node) {
        NodeValue::SubNode(sub_node) => explode(sub_node, level + 1),
        NodeValue::Value(_) => ExplodeResult::None,
    };
    match result {
        ExplodeResult::ExplodeParent {
            replacement,
            right,
            left,
        } => {
            let (self_val, opposite_val) = match side_direction {
                Direction::Left => (left, right),
                Direction::Right => (right, left),
            };
            *side_direction.get_value_mut(node) = NodeValue::Value(replacement);
            // node.right = NodeValue::Value(replacement);
            traverse_down(
                side_direction.get_opposite_value_mut(node),
                opposite_val,
                side_direction,
            );
            ExplodeResult::ExplodeTraverse {
                value: self_val,
                direction: side_direction,
            }
        }
        ExplodeResult::ExplodeTraverse { value, direction } => {
            if direction == side_direction {
                // Propagate further up
                ExplodeResult::ExplodeTraverse { value, direction }
            } else {
                traverse_down(
                    side_direction.get_opposite_value_mut(node),
                    value,
                    side_direction,
                );
                ExplodeResult::DidWork
            }
        }
        // Propagate up the "stop now"
        ExplodeResult::DidWork => ExplodeResult::DidWork,
        // Keep going
        ExplodeResult::None => ExplodeResult::None,
    }
}

fn traverse_down(node_value: &mut NodeValue, to_add: u32, direction: Direction) {
    match node_value {
        NodeValue::Value(v) => *v += to_add,
        NodeValue::SubNode(sub_node) => {
            let next = match direction {
                Direction::Left => &mut sub_node.left,
                Direction::Right => &mut sub_node.right,
            };
            traverse_down(next, to_add, direction);
        }
    }
}

fn split(node: &mut Node) -> bool {
    split_side(node, Direction::Left) || split_side(node, Direction::Right)
}

fn split_side(node: &mut Node, side_direction: Direction) -> bool {
    let result = match side_direction.get_value_mut(node) {
        NodeValue::SubNode(sub_node) => return split(sub_node),
        NodeValue::Value(v) => {
            if *v >= 10 {
                Some(*v)
            } else {
                None
            }
        }
    };
    match result {
        Some(value) => {
            let odd_offset = u32::from(value.is_odd());
            *side_direction.get_value_mut(node) = NodeValue::SubNode(Box::new(Node {
                left: NodeValue::Value(value / 2),
                right: NodeValue::Value(value / 2 + odd_offset),
            }));
            true
        }
        None => false,
    }
}

fn get_magnitude(node: &Node) -> u32 {
    3 * get_magnitude_value(&node.left) + 2 * get_magnitude_value(&node.right)
}

fn get_magnitude_value(node_value: &NodeValue) -> u32 {
    match node_value {
        NodeValue::SubNode(sub_node) => get_magnitude(sub_node),
        NodeValue::Value(v) => *v,
    }
}

fn reduce(node: &mut Node) -> bool {
    // Rely on short circuiting
    explode(node, 0).did_work() || split(node)
}

fn test(s: &str, expected: &str) {
    let mut number = parse(s);
    let expected = parse(expected);
    while reduce(&mut number) {}
    // let result = reduce(&mut number, 0);
    if number != expected {
        println!("{:?} ----> {:?}", s, number);
    }
}

fn longest_matching(a: impl Iterator<Item = u8>, b: impl Iterator<Item = u8>) -> usize {
    a.zip(b).take_while(|(a, b)| a == b).count()
}

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

fn print_diff(before: &str, after: &str) {
    let prefix_len = longest_matching(before.bytes(), after.bytes());
    let suffix_len = longest_matching(before.bytes().rev(), after.bytes().rev());
    println!(
        "{}{}{}{}{}",
        &before[..prefix_len],
        RED,
        &before[prefix_len..(before.len() - suffix_len)],
        RESET,
        &before[(before.len() - suffix_len)..]
    );
    println!(
        "{}{}{}{}{}",
        &after[..prefix_len],
        GREEN,
        &after[prefix_len..(after.len() - suffix_len)],
        RESET,
        &after[(after.len() - suffix_len)..]
    );
}

fn add(a: &Node, b: &Node) -> Node {
    let mut result = Node {
        left: NodeValue::SubNode(Box::new(a.clone())),
        right: NodeValue::SubNode(Box::new(b.clone())),
    };
    // let mut repr = format!("{:?}", result);
    while reduce(&mut result) {
        // let new_repr = format!("{:?}", result);
        // print_diff(&repr, &new_repr);
        // println!("    Reducing {:?}", result);
        // repr = new_repr;
    }
    result
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    // test("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
    // test("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
    // test("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
    // // test(
    // //     "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
    // //     "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
    // // );
    // test(
    //     "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
    //     "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
    // );
    // test(
    //     "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
    //     "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
    // );

    let numbers = input.split('\n').map(parse).collect_vec();
    let mut part1 = numbers.clone();
    let mut result = part1.remove(0);
    for number in part1 {
        result = add(&result, &number);
    }
    println!("Final result: {:?}", result);

    println!("Part 1: {}", get_magnitude(&result));

    let mut max_mag = 0;
    for (i, el1) in numbers.iter().enumerate() {
        for el2 in numbers[i + 1..].iter() {
            max_mag = max(max_mag, get_magnitude(&add(el1, el2)));
            max_mag = max(max_mag, get_magnitude(&add(el2, el1)));
        }
    }

    println!("Part 2: {}", max_mag);
}
