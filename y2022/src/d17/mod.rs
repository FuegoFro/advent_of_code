use itertools::Itertools;
use num::integer::lcm;
use num::Integer;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::iter::{Cycle, Map};
use std::str::{Chars, Split};
use std::time::Instant;
use util::grid::Grid;
use util::point2::{Delta, PointS, PointU};

const WIDTH: usize = 7;
const STABLE_BLOCK_SIZE_HEURISTIC: usize = 5;
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
        if taken_spaces.contains(&PointU::new(origin.x + dx, origin.y - dy)) {
            return true;
        }
    }
    false
}

fn settle(taken_spaces: &mut HashSet<PointU>, origin: &PointU, rock: &Rock) {
    for (dx, dy) in rock.offsets.iter() {
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

fn run_cycles(rocks: &[Rock], wind: &[WindDirection], max_cycles: usize) -> usize {
    let mut rock_index = 0;
    let mut wind_index = 0;
    let mut taken_spaces: HashSet<PointU> = HashSet::new();
    let mut top_taken = 0;
    let mut starting_indices = HashMap::new();

    let mut extra_height: Option<usize> = None;
    let mut i = 0;
    let mut block_sizes: HashMap<usize, usize> = HashMap::new();

    while i < max_cycles {
        if extra_height.is_none() {
            if let Some((previous_cycle, previous_height)) =
                starting_indices.get(&(rock_index, wind_index))
            {
                // Add more height and jump the current cycle
                let block_size = i - previous_cycle;
                let block_height = top_taken - previous_height;
                *block_sizes.entry(block_size).or_default() += 1;
                // Make sure this is actually a stable block size
                if block_sizes.get(&block_size).copied().unwrap_or_default()
                    > STABLE_BLOCK_SIZE_HEURISTIC
                {
                    let remaining = max_cycles - i;
                    let num_blocks_remaining = remaining / block_size;
                    i += num_blocks_remaining * block_size;
                    extra_height = Some(num_blocks_remaining * block_height);
                    continue;
                }
            }
            starting_indices.insert((rock_index, wind_index), (i, top_taken));
        }
        let rock = &rocks[rock_index];
        rock_index = (rock_index + 1) % rocks.len();
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
                        }
                    }
                }
                WindDirection::Right => {
                    if origin.x + rock.width < WIDTH {
                        origin.x += 1;
                        if overlaps(&taken_spaces, &origin, rock) {
                            origin.x -= 1;
                        }
                    }
                }
            }

            // Then go down
            origin.y -= 1;
            if origin.y == 0 || overlaps(&taken_spaces, &origin, rock) {
                origin.y += 1;
                break;
            }
        }
        settle(&mut taken_spaces, &origin, rock);
        top_taken = max(top_taken, origin.y);
        i += 1;
    }
    top_taken + extra_height.unwrap_or_default()
}

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

    let top_taken = run_cycles(&rocks, &wind, 2022);
    println!("Part 1: {}", top_taken);
    let start = Instant::now();
    let top_taken = run_cycles(&rocks, &wind, 1_000_000_000_000);
    let end = Instant::now();
    println!("Part 2: {} ({:?})", top_taken, end - start);
}
