use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const START: &str = "start";
const END: &str = "end";

fn is_small_cave(s: &str) -> bool {
    s.chars().next().unwrap().is_lowercase()
}

/// For now assume no two big caves are next to each other
fn dfs_all_paths_recursive<'container, 'node: 'container>(
    graph: &HashMap<&'node str, Vec<&'node str>>,
    path_so_far: &'container mut Vec<&'node str>,
    visited_small_caves: &'container mut HashSet<&'node str>,
    num_paths: &mut usize,
    has_revisited_small_cave: bool,
) {
    let current_node = path_so_far.last().map_or(START, |node| node);
    if current_node == END {
        // Done with this path!
        *num_paths += 1;
        return;
    } else if current_node == START && !path_so_far.is_empty() {
        // Don't revisit the start
        return;
    }

    let has_visited_this_small_cave = visited_small_caves.contains(&current_node);
    let is_revisiting_small_cave = if !has_visited_this_small_cave {
        // Not visited this yet, can't be revisiting it.
        false
    } else if has_revisited_small_cave {
        // Already revisited, bail.
        return;
    } else {
        // This is our first time revisiting a small cave.
        true
    };

    // Visit all reachable
    // TODO - Can this state (insert then remove) be better encoded in the type system?
    if is_small_cave(&current_node) && !is_revisiting_small_cave {
        visited_small_caves.insert(current_node);
    }
    for reachable in graph[current_node].iter() {
        path_so_far.push(reachable);
        dfs_all_paths_recursive(
            graph,
            path_so_far,
            visited_small_caves,
            num_paths,
            has_revisited_small_cave || is_revisiting_small_cave,
        );
        path_so_far.pop();
    }
    if is_small_cave(&current_node) && !is_revisiting_small_cave {
        visited_small_caves.remove(&current_node);
    }
}

fn dfs_all_paths(graph: &HashMap<&str, Vec<&str>>, allow_two_small_cave_visits: bool) -> usize {
    let mut num_paths = 0;

    dfs_all_paths_recursive(
        graph,
        &mut Vec::new(),
        &mut HashSet::new(),
        &mut num_paths,
        !allow_two_small_cave_visits,
    );
    num_paths
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace("\r", "");

    let graph = input
        .split("\n")
        .map(|line| line.split_once("-").unwrap())
        // Expand with both forwards and backwards links
        // TODO - Turn this to just `.into_iter()` once using 2021 Edition.
        .flat_map(|(a, b)| IntoIterator::into_iter([(a, b), (b, a)]))
        .into_group_map();

    let num_paths = dfs_all_paths(&graph, false);
    println!("Part 1: {}", num_paths);
    let num_paths = dfs_all_paths(&graph, true);
    println!("Part 2: {}", num_paths);
}
