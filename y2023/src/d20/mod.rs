use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

use util::cycle_helpers::FirstCommonCycle;

type Signal = bool;

#[derive(Debug, Clone, Default)]
struct ConjunctionState {
    inputs: HashMap<String, Signal>,
}

#[derive(Debug, Clone)]
enum Module {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(ConjunctionState),
    Output,
}

impl Module {
    fn conjunction_state(&self) -> Option<&ConjunctionState> {
        if let Module::Conjunction(state) = self {
            Some(state)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Network {
    graph: HashMap<String, Vec<String>>,
    modules: HashMap<String, Module>,
}

impl Network {
    fn from_str(input: String) -> Self {
        let mut graph = HashMap::new();
        let mut modules = HashMap::new();
        for line in input.lines() {
            let (name, outs_raw) = line.split_once(" -> ").unwrap();
            let outs = outs_raw.split(", ").map(|o| o.to_owned()).collect_vec();
            let (name, module) = if name == "broadcaster" {
                (name, Module::Broadcaster)
            } else if let Some(name) = name.strip_prefix('%') {
                (name, Module::FlipFlop(false))
            } else if let Some(name) = name.strip_prefix('&') {
                (name, Module::Conjunction(ConjunctionState::default()))
            } else if name == "output" {
                (name, Module::Output)
            } else {
                panic!("Unknown name pattern {}", name);
            };
            graph.insert(name.to_owned(), outs.clone());
            modules.insert(name.to_owned(), module);
        }

        // Post-processing. Initialize conjunctions and setup outputs
        for (src, dsts) in graph.iter() {
            for dst in dsts {
                if let Some(Module::Conjunction(state)) = modules.get_mut(dst) {
                    state.inputs.insert(src.clone(), false);
                }
            }
        }
        for dst in graph.values().flat_map(|vs| vs.iter()) {
            if !modules.contains_key(dst) {
                modules.insert(dst.clone(), Module::Output);
            }
        }

        Self { graph, modules }
    }

    fn handle_signal(
        &mut self,
        signal: Signal,
        source: String,
        target: String,
    ) -> Vec<(Signal, String, String)> {
        let out_signal = match self
            .modules
            .get_mut(&target)
            .unwrap_or_else(|| panic!("Unable to find {}", target))
        {
            Module::Broadcaster => false,
            Module::FlipFlop(prev) => {
                if signal {
                    return Vec::new();
                }
                *prev = !*prev;
                *prev
            }
            Module::Conjunction(state) => {
                *(state.inputs.get_mut(&source).unwrap()) = signal;
                !state.inputs.values().all(|v| *v)
            }
            Module::Output => {
                return Vec::new();
            }
        };

        self.graph[&target]
            .iter()
            .cloned()
            .map(|next| (out_signal, target.clone(), next))
            .collect_vec()
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    // let rx = "output".to_owned();
    let input = include_str!("actual_input.txt").trim().replace('\r', "");
    let rx = "rx".to_owned();

    let mut network = Network::from_str(input);

    let (rx_parent,) = network
        .graph
        .iter()
        .filter(|(_, dsts)| dsts.contains(&rx))
        .map(|(src, _)| src.clone())
        .collect_tuple()
        .unwrap();
    let rx_parent_inputs = network.modules[&rx_parent]
        .conjunction_state()
        .unwrap()
        .inputs
        .keys()
        .cloned()
        .collect::<HashSet<_>>();

    let mut lows = 0;
    let mut highs = 0;
    let mut rx_parent_inputs_high = rx_parent_inputs
        .iter()
        .cloned()
        .map(|n| (n, Vec::new()))
        .collect::<HashMap<_, _>>();
    for i in 0.. {
        // eprintln!("i={}", i);
        if i >= 1000 && rx_parent_inputs_high.values().all(|v| v.len() >= 5) {
            break;
        }
        let mut queue = VecDeque::new();
        let mut high_this_cycle = None;
        let mut step_count = 0;
        queue.push_back((false, "".to_owned(), "broadcaster".to_owned()));
        while let Some((signal, previous, target)) = queue.pop_front() {
            if signal && target == rx_parent {
                rx_parent_inputs_high
                    .get_mut(&previous)
                    .unwrap()
                    .push(i as i128);
                eprintln!("{previous} high on {step_count}");
                high_this_cycle = Some(previous.clone());
                // print_debug = true;
            }
            if let Some(high_this_cycle_name) = high_this_cycle.as_ref() {
                if !signal && target == rx_parent && previous == *high_this_cycle_name {
                    eprintln!("{previous} low on {step_count}");
                    high_this_cycle = None;
                }
            }
            if i < 1000 {
                if signal {
                    highs += 1;
                } else {
                    lows += 1;
                }
            }

            // let should_show = target == rx_parent;
            queue.extend(network.handle_signal(signal, previous, target));
            step_count += 1;
            // if should_show {
            //     eprintln!(
            //         "{:?}",
            //         network.modules[&rx_parent]
            //             .conjunction_state()
            //             .unwrap()
            //             .inputs
            //             .iter()
            //             .collect_vec()
            //     );
            // }
        }
        // if print_debug {
        //     eprintln!(
        //         "{} {:?}",
        //         i,
        //         network.modules[&rx_parent]
        //             .conjunction_state()
        //             .unwrap()
        //             .inputs
        //     );
        // }
        // for name in network.modules[&rx_parent]
        //     .conjunction_state()
        //     .unwrap()
        //     .inputs
        //     .iter()
        //     .filter(|(_, high)| **high)
        //     .map(|(name, _)| name)
        // {
        //     eprintln!("Ended with {} high on {}", name, i);
        //     rx_parent_inputs_high.get_mut(name).unwrap().push(i as i128);
        // }
    }

    let p1 = lows * highs;
    println!("Part 1: {}", p1);

    for (k, v) in rx_parent_inputs_high.iter() {
        println!(
            "{} -> {:?} ({}) -- {:?}",
            k,
            &v[0..std::cmp::min(v.len(), 10)],
            v.len(),
            v.iter().tuple_windows().map(|(a, b)| b - a).collect_vec()
        );
    }
    let p2 = rx_parent_inputs_high
        .values()
        .map(|idxs| (idxs[0], idxs[1]))
        .find_first_common_cycle();

    /*
    tx -> [(65581957384105 - 3768)/ 7537, 11306, 15075, 18844] (5) -- [3769, 3769, 3769, 3769]
    pc -> [(65581957384105 - 3880)/ 7761, 11642, 15523, 19404] (5) -- [3881, 3881, 3881, 3881]
    vd -> [(65581957384105 - 3766)/ 7533, 11300, 15067, 18834] (5) -- [3767, 3767, 3767, 3767]
    nd -> [(65581957384105 - 4018)/ 8037, 12056, 16075, 20094] (5) -- [4019, 4019, 4019, 4019]
    Part 2: 65581957384105

    // 65581957384105 is too low
    // 65581957384106 is too low

    */

    // let network = network_orig;
    // for (src, dsts) in network.graph.iter() {
    //     for dst in dsts {
    //         println!("{} -> {};", src, dst);
    //     }
    // }
    // for (name, module) in network.modules.iter() {
    //     let shape = match module {
    //         Module::Broadcaster => "star",
    //         Module::FlipFlop(_) => "diamond",
    //         Module::Conjunction(_) => "ellipse",
    //         Module::Output => "box",
    //     };
    //     println!("{} [shape={}];", name, shape);
    // }

    // let p2 = first_rx_low_iteration.unwrap();
    println!("Part 2: {:?}", p2);
}
