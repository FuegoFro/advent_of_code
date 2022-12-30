use itertools::Itertools;
use std::collections::HashSet;
use util::p_u32;
use util::point2::{Delta, PointS};

fn calculate_tail(head: PointS, tail: PointS) -> PointS {
    // Handle movement other than full diagonal
    for dir in Delta::NEIGHBORS4.iter() {
        let mid_point = head + dir + dir;
        let perpendicular = dir.rotate_about_origin_deg(90);
        for mult in [-1, 0, 1].into_iter() {
            let potential_tail = mid_point + perpendicular * mult;
            if potential_tail == tail {
                return head + dir;
            }
        }
    }
    // Handle full diagonal
    for dir in Delta::DIAGONALS.iter() {
        if tail == head + dir + dir {
            return head + dir;
        }
    }
    // Otherwise it's fine where it is
    tail
}

fn calculate_tail_positions(instructions: &[(Delta, u32)], rope_length: usize) -> usize {
    let mut rope = vec![PointS::ORIGIN; rope_length];

    let mut tail_squares = HashSet::new();
    tail_squares.insert(*rope.last().unwrap());

    for (dir, count) in instructions.iter() {
        for _ in 0..*count {
            rope[0] += dir;
            // TODO - Couldn't figure out how to use lending-iterator's windows_mut (crate
            //   didn't compile).
            for idx in 1..rope.len() {
                rope[idx] = calculate_tail(rope[idx - 1], rope[idx]);
            }

            tail_squares.insert(*rope.last().unwrap());
        }
    }

    tail_squares.len()
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let instructions = input
        .split('\n')
        .map(|l| {
            let (dir, count) = l.split_once(' ').unwrap();
            let dir = match dir {
                "L" => Delta::LEFT,
                "R" => Delta::RIGHT,
                "U" => Delta::UP,
                "D" => Delta::DOWN,
                _ => panic!("Unknown direction {}", dir),
            };
            let count = p_u32(count);
            (dir, count)
        })
        .collect_vec();

    let pt1 = calculate_tail_positions(&instructions, 2);
    println!("Part 1: {}", pt1);

    let pt2 = calculate_tail_positions(&instructions, 10);
    println!("Part 2: {}", pt2);
}
