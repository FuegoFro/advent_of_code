use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const START: &str = "start";
const END: &str = "end";

fn is_small_cave(s: &str) -> bool {
    s.chars().next().unwrap().is_lowercase()
}

/// For now assume no two big caves are next to each other
/// TODO - Can we make this fully use references?
fn dfs_all_paths_recursive(
    graph: &HashMap<&str, Vec<&str>>,
    path_so_far: &mut Vec<String>,
    visited_small_caves: &mut HashSet<String>,
    all_paths: &mut Vec<Vec<String>>,
    has_revisited_small_cave: bool,
) {
    let current_node = path_so_far
        .last()
        .map_or(START, |node| node.as_str())
        .to_string();
    if current_node == END {
        // Done with this path!
        all_paths.push(path_so_far.clone());
        return;
    } else if current_node == START && !path_so_far.is_empty() {
        // Don't revisit the start
        return;
    }

    // TODO - Can this state (insert then remove) be better encoded in the type system? Can this be a `match`?
    let has_visited_this_small_cave = visited_small_caves.contains(&current_node);
    let is_revisiting_small_cave = if has_visited_this_small_cave {
        if has_revisited_small_cave {
            return;
        } else {
            true
        }
    } else {
        false
    };

    // Visit all reachable
    if is_small_cave(&current_node) && !is_revisiting_small_cave {
        visited_small_caves.insert(current_node.to_string());
    }
    for reachable in graph[current_node.as_str()].iter() {
        path_so_far.push(reachable.to_string());
        dfs_all_paths_recursive(
            graph,
            path_so_far,
            visited_small_caves,
            all_paths,
            has_revisited_small_cave || is_revisiting_small_cave,
        );
        path_so_far.pop();
    }
    if is_small_cave(&current_node) && !is_revisiting_small_cave {
        visited_small_caves.remove(&current_node);
    }
}

fn dfs_all_paths(
    graph: &HashMap<&str, Vec<&str>>,
    all_two_small_cave_visits: bool,
) -> Vec<Vec<String>> {
    let mut all_paths = Vec::new();

    dfs_all_paths_recursive(
        graph,
        &mut Vec::new(),
        &mut HashSet::new(),
        &mut all_paths,
        !all_two_small_cave_visits,
    );
    all_paths
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace("\r", "");

    // TODO - Can this be cleaner?
    let forward = input
        .split("\n")
        .map(|line| line.split_once("-").unwrap())
        .collect_vec();
    let backward = forward.iter().map(|(a, b)| (*b, *a)).collect_vec();
    let graph = forward
        .into_iter()
        .chain(backward.into_iter())
        .into_group_map();

    let all_paths = dfs_all_paths(&graph, false);
    println!("Part 1: {}", all_paths.len());
    let all_paths = dfs_all_paths(&graph, true);
    println!("Part 2: {}", all_paths.len());
}
