use derivative::Derivative;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use recap::Recap;
use serde::Deserialize;
use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};

#[derive(Deserialize, Recap, Debug)]
#[recap(
    regex = r#"Valve (?P<label>\w+) has flow rate=(?P<rate>\d+); tunnels? leads? to valves? (?P<neighbors>.+)"#
)]
struct ValveRaw {
    label: String,
    rate: u32,
    neighbors: Vec<String>,
}

// We use a usize instead of a string since it's *way* more memory efficient and therefore faster.
#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Valve(usize);

impl Display for Valve {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl Valve {
    fn mask(&self) -> u32 {
        1 << self.0
    }
}

fn find_distance(neighbors: &[Vec<Valve>], start: Valve, end: Valve) -> u32 {
    // BFS
    let mut frontier = VecDeque::from([(start, 0)]);
    let mut seen = HashSet::new();
    while let Some((node, distance)) = frontier.pop_front() {
        if seen.contains(&node) {
            continue;
        }
        seen.insert(node);

        if node == end {
            return distance;
        }
        for neighbor in &neighbors[node.0] {
            frontier.push_back((*neighbor, distance + 1));
        }
    }

    panic!("Unable to find path from {} to {}", start, end);
}

struct Valves {
    rates: Vec<u32>,
    neighbors: Vec<Vec<Valve>>,
    starting_valve: Valve,
    pairwise_distances: HashMap<(Valve, Valve), u32>,
}

impl Valves {
    fn from_str(s: &str) -> Self {
        let mut raw_valves = s
            .split('\n')
            .map(|l| {
                let mut valve = l.parse::<ValveRaw>().unwrap();
                valve
                    .neighbors
                    .iter_mut()
                    .for_each(|n| *n = n.trim().to_owned());
                (valve.label, valve.rate, valve.neighbors)
            })
            .collect_vec();

        // Not doing this in the above chain because it makes IntelliJ confused.
        raw_valves.sort_by_key(|(l, r, _)| (*r, l.clone()));
        raw_valves.reverse();

        let label_to_index_mapping = raw_valves
            .iter()
            .enumerate()
            .map(|(index, (label, _, _))| (label.clone(), Valve(index)))
            .collect::<HashMap<_, _>>();

        let rates = raw_valves.iter().map(|(_, rate, _)| *rate).collect_vec();

        let neighbors = raw_valves
            .iter()
            .map(|(_, _, neighbors)| {
                neighbors
                    .iter()
                    .map(|n| label_to_index_mapping[n])
                    .collect_vec()
            })
            .collect_vec();

        let pairwise_distances = (0..raw_valves.len())
            .flat_map(|start| {
                let neighbors = &neighbors;
                (0..raw_valves.len()).map(move |end| {
                    let start = Valve(start);
                    let end = Valve(end);
                    ((start, end), find_distance(neighbors, start, end))
                })
            })
            .collect::<HashMap<_, _>>();

        let starting_valve = label_to_index_mapping[&("AA".to_string())];

        Self {
            rates,
            neighbors,
            starting_valve,
            pairwise_distances,
        }
    }

    fn rate(&self, valve: Valve) -> u32 {
        self.rates[valve.0]
    }

    fn neighbors(&self, valve: Valve) -> impl Iterator<Item = &Valve> {
        self.neighbors[valve.0].iter()
    }
}

#[derive(Clone, Derivative)]
#[derivative(Eq, PartialEq, Hash)]
struct State<const NUM_AGENTS: usize> {
    // #[derivative(PartialEq = "ignore", Hash = "ignore")]
    // actions: Vec<String>,
    open_valves: u32,
    current_valves: [Valve; NUM_AGENTS],
    current_active_agent: usize,
    released_flow: u32,
    time_remaining: u32,
}

impl<const NUM_AGENTS: usize> State<NUM_AGENTS> {
    fn open_current_valve(&self, rate: u32) -> Self {
        let mut new_state = self.clone();
        let valve_open_duration = new_state.time_remaining - 1;
        // new_state.actions.push(format!(
        //     "{}: {} - Opened ({} * {} = {})",
        //     new_state.time_remaining,
        //     new_state.current_valves[new_state.current_active_agent],
        //     rate,
        //     valve_open_duration,
        //     rate * valve_open_duration
        // ));
        new_state.open_valves |= new_state.current_valves[new_state.current_active_agent].mask();
        new_state.released_flow += rate * valve_open_duration;
        new_state.advance()
    }

    fn move_to_valve(&self, valve: Valve) -> Self {
        let mut new_state = self.clone();
        // new_state.actions.push(format!(
        //     "{}: {} - Moved to {}",
        //     new_state.time_remaining,
        //     new_state.current_valves[new_state.current_active_agent],
        //     valve,
        // ));
        new_state.current_valves[new_state.current_active_agent] = valve;
        new_state.advance()
    }

    fn advance(mut self) -> Self {
        self.current_active_agent = (self.current_active_agent + 1) % NUM_AGENTS;
        if self.current_active_agent == 0 {
            self.time_remaining -= 1;
        }
        self
    }

    fn is_valve_open(&self, valve: Valve) -> bool {
        self.open_valves & valve.mask() != 0
    }
}

fn enqueue_state<const NUM_AGENTS: usize>(
    frontier: &mut PriorityQueue<State<NUM_AGENTS>, u32>,
    valves: &Valves,
    state: State<NUM_AGENTS>,
) {
    let max_remaining_flow_upper_bound: u32 = valves
        .rates
        .iter()
        .enumerate()
        .map(|(index, rate)| (Valve(index), rate))
        .filter(|(valve, rate)| **rate > 0 && !state.is_valve_open(*valve))
        .map(|(valve, rate)| {
            let closest_distance = state
                .current_valves
                .iter()
                .map(|other| valves.pairwise_distances[&(*other, valve)])
                .min()
                .unwrap();
            rate * state.time_remaining.saturating_sub(1 + closest_distance)
        })
        .sum();
    let priority = state.released_flow + max_remaining_flow_upper_bound;
    frontier.push(state, priority);
}

fn do_a_star<const NUM_AGENTS: usize>(valves: &Valves, total_time: u32) -> u32 {
    // Ye olde A*
    let mut visited = HashSet::new();
    let mut frontier = PriorityQueue::new();
    frontier.push(
        State {
            // actions: vec![],
            open_valves: 0,
            current_valves: [valves.starting_valve; NUM_AGENTS],
            current_active_agent: 0,
            released_flow: 0,
            time_remaining: total_time,
        },
        0,
    );

    while let Some((state, _)) = frontier.pop() {
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state.clone());

        if state.time_remaining == 0 {
            // for action in state.actions.iter() {
            //     dbg!(action);
            // }
            return state.released_flow;
        }

        let current_valve = state.current_valves[state.current_active_agent];

        // Don't turn on the current valve, just go to another one
        for neighbor in valves.neighbors(current_valve) {
            enqueue_state(&mut frontier, valves, state.move_to_valve(*neighbor));
        }

        let rate = valves.rate(current_valve);
        if rate > 0 && state.time_remaining > 0 && !state.is_valve_open(current_valve) {
            enqueue_state(&mut frontier, valves, state.open_current_valve(rate));
        }
    }

    panic!("Unable to find solution!");
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let valves = Valves::from_str(&input);

    let max_pressure_pt1 = do_a_star::<1>(&valves, 30);
    println!("Part 1: {}", max_pressure_pt1);

    let max_pressure_pt2 = do_a_star::<2>(&valves, 26);
    println!("Part 2: {}", max_pressure_pt2);
}
