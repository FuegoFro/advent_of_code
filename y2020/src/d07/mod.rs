use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let bag_graph = input
        .split('\n')
        .map(|l| {
            lazy_static! {
                static ref RE_RULE: Regex =
                    Regex::new(r"^(?P<containing_color>.+) bags contain (?P<contained_bags>.+)\.$")
                        .unwrap();
            }
            // IntelliJ doesn't understand this without this alias.
            let re_rule: &Regex = &RE_RULE;
            let caps = re_rule.captures(l).expect(l);
            let containing_color = caps.name("containing_color").unwrap().as_str();
            let contained_bags_raw = caps.name("contained_bags").unwrap().as_str();
            let contained_bags = if contained_bags_raw == "no other bags" {
                Vec::new()
            } else {
                contained_bags_raw
                    .split(", ")
                    .map(|contained_bag| {
                        lazy_static! {
                            static ref RE_CONTAINED_BAG: Regex =
                                Regex::new(r"^(?P<bag_count>\d+) (?P<bag_color>.+) bags?$")
                                    .unwrap();
                        }
                        // IntelliJ doesn't understand this without this alias.
                        let re_contained_bag: &Regex = &RE_CONTAINED_BAG;
                        let caps = re_contained_bag
                            .captures(contained_bag)
                            .expect(contained_bag);
                        let bag_count = caps
                            .name("bag_count")
                            .unwrap()
                            .as_str()
                            .parse::<u32>()
                            .unwrap();
                        let bag_color = caps.name("bag_color").unwrap().as_str();
                        (bag_count, bag_color)
                    })
                    .collect()
            };
            (containing_color, contained_bags)
        })
        .collect::<HashMap<_, _>>();

    part_1(&bag_graph);
    part_2(&bag_graph);
}

fn part_1(bag_graph: &HashMap<&str, Vec<(u32, &str)>>) {
    let mut reverse_bag_graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (containing_bag, contained_bags) in bag_graph.iter() {
        for (_, contained_bag_color) in contained_bags {
            reverse_bag_graph
                .entry(*contained_bag_color)
                .or_insert_with(HashSet::new)
                .insert(*containing_bag);
        }
    }

    // Traverse graph
    let mut frontier = vec!["shiny gold"];
    let mut seen = HashSet::new();
    while let Some(next) = frontier.pop() {
        if seen.contains(&next) {
            continue;
        }
        seen.insert(next);

        if let Some(containing_bags) = reverse_bag_graph.get(next) {
            for containing in containing_bags {
                frontier.push(containing);
            }
        }
    }

    eprintln!("seen = {:?}", seen);
    eprintln!("{}", seen.len() - 1);
}

fn part_2(bag_graph: &HashMap<&str, Vec<(u32, &str)>>) {
    let mut count_cache = HashMap::new();
    let inner_count = count_contained("shiny gold", bag_graph, &mut count_cache);
    println!("{}", inner_count);
}

fn count_contained<'cache, 'data: 'cache>(
    color: &'data str,
    bag_graph: &HashMap<&'data str, Vec<(u32, &'data str)>>,
    count_cache: &'cache mut HashMap<&'data str, u32>,
) -> u32 {
    if let Some(existing) = count_cache.get(color) {
        *existing
    } else {
        let value = bag_graph[color]
            .iter()
            .map(|&(inner_count, inner_color): &(u32, &str)| {
                inner_count * (1 + count_contained(inner_color, bag_graph, count_cache))
            })
            .sum();
        count_cache.insert(color, value);
        value
    }
}
