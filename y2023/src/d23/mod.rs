use std::cmp::max;
use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use util::grid::{Grid, Neighbors};
use util::impl_debug_serde;
use util::point2::{DeltaU, Point, PointU};

impl_debug_serde!(Cell);
#[derive(PartialEq, Serialize, Deserialize)]
enum Cell {
    #[serde(rename = "#")]
    Wall,
    #[serde(rename = ".")]
    Path,
    #[serde(rename = "^")]
    Up,
    #[serde(rename = ">")]
    Right,
    #[serde(rename = "v")]
    Down,
    #[serde(rename = "<")]
    Left,
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let grid = Grid::<Cell>::from_serde_chars(input);

    let start = (0..grid.width())
        .map(|x| PointU::new(x, 0))
        .find(|p| grid[*p] == Cell::Path)
        .unwrap();
    let end = (0..grid.width())
        .map(|x| PointU::new(x, grid.height() - 1))
        .find(|p| grid[*p] == Cell::Path)
        .unwrap();

    let p1 = get_longest_path(&grid, start, end, false);
    println!("Part 1: {}", p1);
    let p2 = get_longest_path_undirected(&grid, start, end);
    println!("Part 2: {}", p2);
}

fn get_longest_path(
    grid: &Grid<Cell>,
    start: Point<usize>,
    end: Point<usize>,
    slopes_any_dir: bool,
) -> usize {
    let mut longest_path = 0;
    let mut queue = vec![(start, BTreeSet::new())];
    while let Some((current, mut visited)) = queue.pop() {
        if current == end {
            longest_path = max(visited.len(), longest_path);
            continue;
        }
        visited.insert((current.x, current.y));
        if grid[current] == Cell::Path || slopes_any_dir {
            for next in grid
                .neighbors_with_values(current, Neighbors::Four)
                .filter(|(p, c)| **c != Cell::Wall && !visited.contains(&(p.x, p.y)))
                .map(|(p, _)| p)
            {
                queue.push((next, visited.clone()));
            }
        } else {
            let delta = match grid[current] {
                Cell::Up => DeltaU::UP,
                Cell::Right => DeltaU::RIGHT,
                Cell::Down => DeltaU::DOWN,
                Cell::Left => DeltaU::LEFT,
                _ => unreachable!(),
            };
            let next = current + delta;
            if grid[next] != Cell::Wall && !visited.contains(&(next.x, next.y)) {
                queue.push((next, visited.clone()));
            }
        }
    }

    longest_path
}

fn simplified_graph(
    grid: &Grid<Cell>,
    initial: PointU,
    end: PointU,
) -> (HashMap<u32, Vec<(u32, u32)>>, u32, u32) {
    // Break map into segments
    // half inclusive [start, end) pairs with segment length.
    // s -> [(e, l), ...]
    // has dupes (rev)

    let mut point_to_idx = HashMap::new();

    fn make_connection(
        connections: &mut Vec<(u32, (u32, u32))>,
        point_to_idx: &mut HashMap<PointU, u32>,
        a: PointU,
        b: PointU,
        dist: u32,
    ) {
        let len = point_to_idx.len() as _;
        let a = *point_to_idx.entry(a).or_insert(len);
        let len = point_to_idx.len() as _;
        let b = *point_to_idx.entry(b).or_insert(len);
        connections.push((a, (b, dist)));
        connections.push((b, (a, dist)));
    }

    let mut visited = HashSet::new();
    visited.insert(initial);
    let initial_next = grid
        .neighbors_with_values(initial, Neighbors::Four)
        .find(|(_, c)| **c != Cell::Wall)
        .unwrap()
        .0;

    let mut queue = vec![(initial, initial_next)];
    let mut connections = Vec::new();
    while let Some((start, mut current)) = queue.pop() {
        for len in 1.. {
            visited.insert(current);
            let nexts = grid
                .neighbors_with_values(current, Neighbors::Four)
                .filter(|(p, c)| **c != Cell::Wall && !visited.contains(p))
                .map(|(p, _)| p)
                .collect_vec();
            match nexts.len() {
                2.. => {
                    // Junction, mark the connection, enqueue each of the nexts
                    make_connection(&mut connections, &mut point_to_idx, start, current, len);
                    for next in nexts {
                        queue.push((current, next));
                    }
                    break;
                }
                1 => {
                    // Continue onward
                    current = nexts[0];
                }
                0 => {
                    // End of a dead-end, mark the connection
                    make_connection(&mut connections, &mut point_to_idx, start, current, len);
                    break;
                }
            }
        }
    }

    let graph = connections.into_iter().into_group_map();
    (graph, point_to_idx[&initial], point_to_idx[&end])
}

fn get_longest_path_undirected(grid: &Grid<Cell>, start: Point<usize>, end: Point<usize>) -> u32 {
    let (graph, start, end) = simplified_graph(grid, start, end);

    let mut longest_path = 0;
    let mut queue = vec![(start, 0, [start].into_iter().collect::<BTreeSet<_>>())];
    while let Some((current, len, mut visited)) = queue.pop() {
        if current == end {
            longest_path = max(len, longest_path);
            continue;
        }
        visited.insert(current);
        for (next, dist) in graph[&current].iter().filter(|(n, _)| !visited.contains(n)) {
            queue.push((*next, len + dist, visited.clone()));
        }
    }

    longest_path
}
