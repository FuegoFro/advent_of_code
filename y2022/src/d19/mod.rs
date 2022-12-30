#![allow(clippy::needless_question_mark)]
use itertools::Itertools;
use priority_queue::PriorityQueue;
use recap::Recap;
use serde::Deserialize;
use std::cmp::max;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::panic;
use std::time::Instant;
use util::{p_u32, split_once};

#[derive(Clone, Copy, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Resource {
    fn from_str(s: &str) -> Self {
        match s {
            "ore" => Self::Ore,
            "clay" => Self::Clay,
            "obsidian" => Self::Obsidian,
            "geode" => Self::Geode,
            _ => panic!("Unknown resource {:?}", s),
        }
    }

    fn index(&self) -> usize {
        match self {
            Resource::Ore => 0,
            Resource::Clay => 1,
            Resource::Obsidian => 2,
            Resource::Geode => 3,
        }
    }
}

const NUM_RESOURCES: usize = std::mem::variant_count::<Resource>();

#[derive(Deserialize, Recap)]
#[recap(
    regex = r#"Each (?P<product>\w+) robot costs (?P<cost1>\d+ \w+)( and (?P<cost2>\d+ \w+))?"#
)]
struct RobotRaw {
    product: String,
    cost1: String,
    cost2: Option<String>,
}

#[derive(Debug)]
struct Robot {
    product: Resource,
    cost: Vec<(u32, Resource)>,
}

impl Robot {
    fn from_str(s: &str) -> Self {
        let raw = s.parse::<RobotRaw>().unwrap();
        let cost = [Some(raw.cost1), raw.cost2]
            .into_iter()
            .flatten()
            .map(|cost_str| {
                let (count, resource) = cost_str.split_once(' ').unwrap();
                (p_u32(count), Resource::from_str(resource))
            })
            .collect_vec();
        Robot {
            product: Resource::from_str(&raw.product),
            cost,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    time_remaining: u32,
    resources_available: [u32; NUM_RESOURCES],
    robots_available: [u32; NUM_RESOURCES],
}

impl State {
    fn initial(initial_time: u32) -> State {
        let mut robots_available = [0; NUM_RESOURCES];
        robots_available[Resource::Ore.index()] = 1;
        State {
            time_remaining: initial_time,
            resources_available: [0; NUM_RESOURCES],
            robots_available,
        }
    }

    fn advance_time(mut self) -> Self {
        self.time_remaining -= 1;
        for (index, count) in self.robots_available.iter().enumerate() {
            self.resources_available[index] += count
        }
        self
    }

    fn build_robot(&self, robot: &Robot) -> Option<Self> {
        let mut new_state = self.clone();
        for (count, resource) in &robot.cost {
            let value = &mut new_state.resources_available[resource.index()];
            if let Some(new_value) = value.checked_sub(*count) {
                *value = new_value;
            } else {
                return None;
            }
        }
        let mut new_state = new_state.advance_time();
        new_state.robots_available[robot.product.index()] += 1;
        Some(new_state)
    }
}

fn enqueue(frontier: &mut PriorityQueue<State, u32>, state: State) {
    // Max possible additional geodes is (current_geode_robots) + (current_geode_robots + 1) + ...
    // + (current_geode_robots + time_remaining)
    // We can do that quickly with the formula to sum an arithmetic sequence:
    // https://study.com/learn/lesson/sum-of-arithmetic-sequence-formula-examples-what-is-arithmetic-sequence.html#section---SumOfAnArithmeticSequence
    let n = state.time_remaining;
    let a = state.robots_available[Resource::Geode.index()];
    // d == 1
    let max_additional_geodes = if n == 0 { 0 } else { (n * (2 * a + n - 1)) / 2 };

    let priority = state.resources_available[Resource::Geode.index()] + max_additional_geodes;
    frontier.push(state, priority);
}

fn get_max_geodes_per_blueprint(blueprints: &[Vec<Robot>], initial_time: u32) -> Vec<u32> {
    blueprints
        .iter()
        .map(|robots| {
            let start = Instant::now();
            let mut frontier = PriorityQueue::new();
            enqueue(&mut frontier, State::initial(initial_time));
            let mut visited = HashSet::new();
            while let Some((node, _)) = frontier.pop() {
                if node.time_remaining == 0 {
                    let quality_level = node.resources_available[Resource::Geode.index()];
                    dbg!(Instant::now() - start);
                    dbg!(quality_level);
                    return quality_level;
                }
                if visited.contains(&node) {
                    continue;
                }

                enqueue(&mut frontier, node.clone().advance_time());
                for robot in robots {
                    if let Some(next) = node.build_robot(robot) {
                        enqueue(&mut frontier, next);
                    }
                }
                // Do this at the end so we don't have to clone the node.
                visited.insert(node);
            }
            panic!("Unable to find path to the end???")
        })
        .collect_vec()
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let blueprints = input
        .split('\n')
        .map(|l| {
            let (_, l) = l.split_once(": ").unwrap();
            l.strip_suffix('.')
                .unwrap()
                .split('.')
                .map(Robot::from_str)
                .collect_vec()
        })
        .collect_vec();

    let quality_levels = get_max_geodes_per_blueprint(&blueprints, 24)
        .into_iter()
        .enumerate()
        .map(|(i, ql)| (i + 1) * ql as usize)
        .sum::<usize>();
    println!("Part 1: {}", quality_levels);

    let max_geodes_mult = get_max_geodes_per_blueprint(&blueprints[..3], 32)
        .into_iter()
        .product::<u32>();
    println!("Part 2: {}", max_geodes_mult);
}
