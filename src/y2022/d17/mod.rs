use crate::util::grid::Grid;
use crate::util::point2::{Delta, PointS, PointU};
use itertools::Itertools;
use num::integer::lcm;
use num::Integer;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::iter::{Cycle, Map};
use std::str::{Chars, Split};
use std::time::Instant;

// Only check side of rock in direction of motion
// Use circular buffer to hold data.
//  Maybe data is the generation, then we check if it equals the current generation

const WIDTH: usize = 7;
const ROCK_PATTERNS: &str = r"
####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##
";

#[derive(Debug)]
struct Rock {
    offsets: Vec<(usize, usize)>,
    height: usize,
    width: usize,
}

impl Rock {
    fn from_str(s: &str) -> Self {
        let grid = Grid::from_str(s, "\n", None, |c| match c {
            "#" => true,
            "." => false,
            _ => panic!("Unknown char {:?}", c),
        });
        let offsets = grid
            .iter_with_points()
            .filter(|(_, is_filled)| **is_filled)
            .map(|(point, _)| {
                let delta = point - PointU::ORIGIN;
                (delta.dx as usize, delta.dy as usize)
            })
            .collect_vec();
        Self {
            offsets,
            height: grid.height(),
            width: grid.width(),
        }
    }
}

enum WindDirection {
    Left,
    Right,
}

fn overlaps(taken_spaces: &HashSet<PointU>, origin: &PointU, rock: &Rock) -> bool {
    for (dx, dy) in rock.offsets.iter() {
        // PointU::new(origin.x + dx, origin.y + dy);
        if taken_spaces.contains(&PointU::new(origin.x + dx, origin.y - dy)) {
            return true;
        }
    }
    false
    // rock.offsets
    //     .iter()
    //     .any(|(dx, dy)| taken_spaces.contains(&PointU::new(origin.x + dx, origin.y + dy)))
}

fn settle(taken_spaces: &mut HashSet<PointU>, origin: &PointU, rock: &Rock) {
    for (dx, dy) in rock.offsets.iter() {
        // let point = origin + offset;
        // PointU::new(origin.x + dx, origin.y + dy);
        taken_spaces.insert(PointU::new(origin.x + dx, origin.y - dy));
    }
}

fn render(top: usize, taken: &HashSet<PointU>) {
    let mut grid = Grid::empty(7, top);
    for x in 0..7 {
        for y in 1..=top {
            grid[PointU::new(x, top - y)] = if taken.contains(&PointU::new(x, y)) {
                '#'
            } else {
                '.'
            }
        }
    }
    dbg!(grid);
}

fn run_cycles(
    rocks: &[Rock],
    wind: &[WindDirection],
    max_cycles: usize,
) -> (usize, HashSet<PointU>, usize) {
    let mut rock_index = 0;
    let mut wind_index = 0;
    // let mut rocks = rocks.iter().cycle();
    // let mut wind = wind.iter().cycle();
    let mut taken_spaces: HashSet<PointU> = HashSet::new();
    let mut top_taken = 0;
    // let mut report_time = Instant::now();
    let mut starting_indices = HashMap::new();

    let mut extra_height: Option<usize> = None;
    let mut i = 0;
    let mut dupes_seen = 0;
    let mut block_sizes: HashMap<usize, usize> = HashMap::new();
    let mut block_heights: HashMap<usize, usize> = HashMap::new();

    while i < max_cycles {
        if extra_height.is_none() {
            if let Some((previous_cycle, previous_height)) =
                starting_indices.get(&(rock_index, wind_index))
            {
                // println!(
                //     "Saw {:?} at {} and {}",
                //     (rock_index, wind_index),
                //     previous_cycle,
                //     i
                // );
                // Add more height and jump the current cycle
                let block_size = i - previous_cycle;
                let block_height = top_taken - previous_height;
                *block_sizes.entry(block_size).or_default() += 1;
                *block_heights.entry(block_height).or_default() += 1;
                if block_sizes.get(&block_size).copied().unwrap_or_default() > 5 {
                    let remaining = max_cycles - i;
                    let (num_blocks_remaining, tmp) = remaining.div_mod_floor(&block_size);
                    dbg!(i);
                    dbg!(top_taken);
                    dbg!(previous_cycle);
                    dbg!(previous_height);
                    dbg!(remaining);
                    dbg!(block_size);
                    dbg!(block_height);
                    dbg!(num_blocks_remaining);
                    i += num_blocks_remaining * block_size;
                    extra_height = Some(num_blocks_remaining * block_height);
                    dbg!(i);
                    dbg!(extra_height);
                    dbg!(max_cycles - i);
                    dbg!(tmp);
                    // *block_sizes.entry(block_size).or_default() += 1;
                    // *block_heights.entry(block_height).or_default() += 1;
                    // dupes_seen += 1;
                    // if dupes_seen >= (5 * block_size) {
                    //     dbg!(block_sizes);
                    //     dbg!(block_heights);
                    //     break;
                    // }
                    // 20
                    // 2 A
                    // 7 B
                    // + 10
                    // continue;
                }
            }
            starting_indices.insert((rock_index, wind_index), (i, top_taken));
        }
        // if i % 1_000_000 == 0 {
        //     let next_time = Instant::now();
        //     println!(
        //         "{} - {:?} ({}, {})",
        //         i,
        //         next_time - report_time,
        //         rock_index,
        //         wind_index
        //     );
        //     report_time = next_time;
        // }
        let rock = &rocks[rock_index];
        rock_index = (rock_index + 1) % rocks.len();
        // println!("New rock {} - {:?}", top_taken, rock.offsets);
        let mut origin = PointU::new(2, top_taken + 3 + rock.height);
        loop {
            // First go left/right
            let wind_direction = &wind[wind_index];
            wind_index = (wind_index + 1) % wind.len();
            match wind_direction {
                WindDirection::Left => {
                    if origin.x > 0 {
                        origin.x -= 1;
                        if overlaps(&taken_spaces, &origin, rock) {
                            origin.x += 1;
                            //     println!("Cannot move left (overlap)")
                            // } else {
                            //     println!("Moved left")
                        }
                        // } else {
                        //     println!("Cannot move left (edge)")
                    }
                }
                WindDirection::Right => {
                    if origin.x + rock.width < WIDTH {
                        origin.x += 1;
                        if overlaps(&taken_spaces, &origin, rock) {
                            origin.x -= 1;
                            //     println!("Cannot move right (overlap)")
                            // } else {
                            //     println!("Moved right")
                        }
                        // } else {
                        //     println!("Cannot move right (edge)")
                    }
                }
            }

            // Then go down
            origin.y -= 1;
            if origin.y == 0 || overlaps(&taken_spaces, &origin, rock) {
                origin.y += 1;
                // println!("Hit bottom");
                break;
                // } else {
                //     println!("Moved down");
            }
        }
        settle(&mut taken_spaces, &origin, rock);
        top_taken = max(top_taken, origin.y);
        // println!("new top taken {}", top_taken);
        i += 1;
    }
    // render(top_taken, &taken_spaces);
    dbg!(top_taken);
    (
        top_taken + extra_height.unwrap_or_default(),
        taken_spaces,
        max_cycles,
    )
}

// fn run_simulation(rocks: &[Rock], wind: &[WindDirection], num_cycles: usize) -> usize {
//     let (initial_height, _, cycles_taken) = run_cycles(rocks, wind, num_cycles);
//     if cycles_taken == num_cycles {
//         return initial_height;
//     }
//
//     let block_size = cycles_taken;
//     let (num_blocks, remaining_cycles) = num_cycles.div_mod_floor(&block_size);
//     dbg!(block_size);
//     dbg!(num_blocks);
//     dbg!(remaining_cycles);
//
//     let (remaining_height, _, _) = run_cycles(rocks, wind, remaining_cycles);
//
//     initial_height * num_blocks + remaining_height
//
//     // // First do `block_size` cycles
//     // // Assert/assume stable packing?
//     // // Do `remaining_cycles` cycles
//     //
//     // let block_height = if num_blocks > 0 {
//     //     let (top_taken, _) =
//     //     let block_height = num_blocks * top_taken;
//     //     println!(
//     //         "block - size={}, {} * {} = {}",
//     //         block_size, num_blocks, top_taken, block_height
//     //     );
//     //     block_height
//     // } else {
//     //     0
//     // };
//     //
//     // let (remaining_cycle_height, _) = run_cycles(rocks, wind, remaining_cycles);
//     // println!(
//     //     "remaining - size={}, {}",
//     //     remaining_cycles, remaining_cycle_height
//     // );
//     //
//     // block_height + remaining_cycle_height
// }

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let rocks = ROCK_PATTERNS
        .trim()
        .split("\n\n")
        .map(Rock::from_str)
        .collect_vec();

    let wind = input
        .chars()
        .map(|c| match c {
            '>' => WindDirection::Right,
            '<' => WindDirection::Left,
            _ => panic!("Unknown char {:?}", c),
        })
        .collect_vec();

    let top_taken = run_cycles(&rocks, &wind, 2022).0;
    println!("Part 1: {}", top_taken);
    let top_taken = run_cycles(&rocks, &wind, 3160).0;
    println!("Part 1: {}", top_taken);
    // let top_taken = run_cycles(&rocks, &wind, 40).0;
    // println!("Part 1: {}", top_taken);
    // let top_taken = run_cycles(&rocks, &wind, 80).0;
    // println!("Part 1: {}", top_taken);
    // let top_taken = run_cycles(&rocks, &wind, 120).0;
    // println!("Part 1: {}", top_taken);
    // let top_taken = run_cycles(&rocks, &wind, 160).0;
    // println!("Part 1: {}", top_taken);
    // let top_taken = run_cycles(&rocks, &wind, 200).0;
    // println!("Part 1: {}", top_taken);
    let start = Instant::now();
    let top_taken = run_cycles(&rocks, &wind, 1_000_000_000_000).0;
    let end = Instant::now();
    println!("Part 2: {} ({:?})", top_taken, end - start);
}
