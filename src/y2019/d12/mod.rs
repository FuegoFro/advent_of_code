use crate::util::p_i32;
use itertools::Itertools;
use num::Integer;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

struct Planet {
    position: [i32; 3],
    velocity: [i32; 3],
}

impl Planet {
    fn from_packed(packed: &str) -> Self {
        lazy_static! {
            static ref RE_POS: Regex = Regex::new(r"^<x=([^,]+), y=([^,]+), z=([^,]+)>$").unwrap();
        }
        let re_pos: &Regex = &RE_POS;
        let caps = re_pos.captures(packed).unwrap();
        Planet {
            position: [p_i32(&caps[1]), p_i32(&caps[2]), p_i32(&caps[3])],
            velocity: [0; 3],
        }
    }
}

fn calc_energy(xyz: &[i32; 3]) -> i32 {
    xyz.iter().map(|c| c.abs()).sum()
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    // let steps = 100;
    let input = include_str!("actual_input.txt").trim();
    let steps = 1000;

    pt1(input, steps);
    pt2(input);
}

fn pt1(input: &str, steps: i32) {
    let mut planets = input.lines().map(Planet::from_packed).collect_vec();
    for _ in 0..steps {
        // Update velocities
        for cur_idx in 0..planets.len() {
            for oth_idx in 0..planets.len() {
                if cur_idx == oth_idx {
                    continue;
                }
                for axis in [0_usize, 1, 2].iter() {
                    let cur_pos = planets[cur_idx].position[*axis];
                    let oth_pos = planets[oth_idx].position[*axis];
                    let velocity_change = match cur_pos.cmp(&oth_pos) {
                        Ordering::Less => 1,
                        Ordering::Equal => 0,
                        Ordering::Greater => -1,
                    };
                    planets.get_mut(cur_idx).unwrap().velocity[*axis] += velocity_change;
                }
            }
        }
        // Update positions
        for planet in planets.iter_mut() {
            for axis in [0_usize, 1, 2].iter() {
                planet.position[*axis] += planet.velocity[*axis];
            }
        }
    }

    let total_energy = planets
        .iter()
        .map(|planet| calc_energy(&planet.position) * calc_energy(&planet.velocity))
        .sum::<i32>();
    println!("{}", total_energy);
}

fn state_hash(positions: &mut Vec<i32>, velocities: &mut Vec<i32>) -> u64 {
    let mut hasher = DefaultHasher::new();
    positions.hash(&mut hasher);
    velocities.hash(&mut hasher);
    hasher.finish()
}

fn run_axis_to_repetition(mut positions: Vec<i32>) -> (i32, i32) {
    let mut velocities = vec![0; positions.len()];
    let mut seen: HashMap<u64, i32> = HashMap::new();

    seen.insert(state_hash(&mut positions, &mut velocities), 0);
    for iteration in 1.. {
        // Update velocities
        for cur_idx in 0..positions.len() {
            for oth_idx in 0..positions.len() {
                if cur_idx == oth_idx {
                    continue;
                }
                let cur_pos = positions[cur_idx];
                let oth_pos = positions[oth_idx];
                let velocity_change = match cur_pos.cmp(&oth_pos) {
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                };
                velocities[cur_idx] += velocity_change;
            }
        }
        // Update positions
        for (position, velocity) in positions.iter_mut().zip_eq(velocities.iter()) {
            *position += velocity;
        }
        let current_state_hash = state_hash(&mut positions, &mut velocities);
        if let Some(first_seen) = seen.get(&current_state_hash) {
            return (*first_seen, iteration);
        } else {
            seen.insert(current_state_hash, iteration);
        }
    }

    panic!("Shouldn't be able to get here");
}

fn pt2(input: &str) {
    let initial_planets = input.lines().map(Planet::from_packed).collect_vec();
    let (first_x, second_x) =
        run_axis_to_repetition(initial_planets.iter().map(|p| p.position[0]).collect_vec());
    let (first_y, second_y) =
        run_axis_to_repetition(initial_planets.iter().map(|p| p.position[1]).collect_vec());
    let (first_z, second_z) =
        run_axis_to_repetition(initial_planets.iter().map(|p| p.position[2]).collect_vec());
    println!(
        "{:?}",
        (first_x, second_x, first_y, second_y, first_z, second_z)
    );
    assert_eq!(first_x, 0);
    assert_eq!(first_y, 0);
    assert_eq!(first_z, 0);
    let first_repetition = (second_x as u64)
        .lcm(&(second_y as u64))
        .lcm(&(second_z as u64));
    println!("{}", first_repetition);
}
