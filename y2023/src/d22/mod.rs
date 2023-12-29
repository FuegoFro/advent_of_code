use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet};
use util::p_i32;
use util::point3::{BoundingBox, Delta3, Point3};

fn str_to_point3(s: &str) -> Point3 {
    let (x, y, z) = s.split(',').map(p_i32).collect_tuple().unwrap();
    Point3::new(x, y, z)
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let mut blocks = input
        .lines()
        .map(|l| {
            let (start, end) = l.split_once('~').unwrap();
            let start = str_to_point3(start);
            let end = str_to_point3(end);
            assert!(
                start.x <= end.x && start.y <= end.y && start.z <= end.z,
                "{}",
                l
            );
            BoundingBox::new(start, end + Delta3::new(1, 1, 1))
        })
        .sorted_by_key(|b| b.start.z)
        .collect_vec();

    // If a block has an entry, it is stable. If the corresponding vec is empty it's on the ground
    let mut supported_by = HashMap::<usize, Vec<usize>>::new();

    for block_idx in 0..blocks.len() {
        // For each block that's unsupported, move down until it hits at least one other block or the ground
        // (start.z == 1)
        loop {
            // eprintln!("Moving {} down 1", block_idx);
            blocks[block_idx].start += Delta3::Z_NEG;
            blocks[block_idx].end += Delta3::Z_NEG;
            if blocks[block_idx].start.z == 0 {
                // Intersected the ground, finish and undo
                // eprintln!("  {} in ground", block_idx);

                supported_by.insert(block_idx, Vec::new());
                break;
            }
            let intersecting = (0..blocks.len())
                .filter(|other_idx| {
                    *other_idx != block_idx
                        && !blocks[*other_idx].intersect(&blocks[block_idx]).is_empty()
                })
                .collect_vec();
            if !intersecting.is_empty() {
                // Intersected with other blocks, finish and undo
                // eprintln!("  {} in {:?}", block_idx, intersecting);

                supported_by.insert(block_idx, intersecting);
                break;
            }
        }
        // eprintln!("Moving {} back up 1", block_idx);

        // Undo the last movement since it was invalid
        blocks[block_idx].start += Delta3::Z_POS;
        blocks[block_idx].end += Delta3::Z_POS;
    }

    let mut supporting = supported_by
        .iter()
        .flat_map(|(supported, supporters)| {
            supporters
                .iter()
                .map(move |supporter| (*supporter, *supported))
        })
        .into_group_map();

    // Can dissolve if anything we're supporting is also supported by something else
    let can_dissolve = (0..blocks.len())
        .filter(|idx| {
            supporting
                .entry(*idx)
                .or_default()
                .iter()
                .all(|supported_idx| supported_by[supported_idx].len() > 1)
        })
        .collect_vec();

    let p1 = can_dissolve.len();

    println!("Part 1: {}", p1);

    let p2 = (0..blocks.len())
        .map(|initial_idx| {
            if initial_idx % 100 == 0 {
                println!("{}", initial_idx);
            }
            let mut moved = HashSet::new();
            let mut to_check = PriorityQueue::new();
            fn enqueue(queue: &mut PriorityQueue<usize, i32>, blocks: &[BoundingBox], idx: usize) {
                queue.push(idx, blocks[idx].start.z);
            }

            // Initial setup
            moved.insert(initial_idx);
            for initial_supports in supporting[&initial_idx].iter() {
                enqueue(&mut to_check, &blocks, *initial_supports);
            }

            while let Some((idx, _)) = to_check.pop() {
                if moved.contains(&idx) {
                    continue;
                }
                let all_supports_moved = supported_by[&idx]
                    .iter()
                    .all(|supported_by_idx| moved.contains(supported_by_idx));
                if !all_supports_moved {
                    continue;
                }
                moved.insert(idx);
                for next in supporting[&idx].iter() {
                    enqueue(&mut to_check, &blocks, *next);
                }
            }
            moved.len() - 1
        })
        .sum::<usize>();

    // 69872 too high
    println!("Part 2: {}", p2);
}
