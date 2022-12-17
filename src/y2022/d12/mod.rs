use crate::util::grid::{Grid, Neighbors};
use crate::util::point2::PointU;
use std::collections::{HashSet, VecDeque};

const START_SENTINEL: u8 = 100;
const END_SENTINEL: u8 = 101;

fn do_bfs(start: PointU, end: PointU, grid: &Grid<u8>) -> Option<u32> {
    // Classic BFS
    let mut visited = HashSet::new();
    let mut frontier = VecDeque::from([(start, 0u32)]);
    while let Some((current, len)) = frontier.pop_front() {
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);
        if current == end {
            return Some(len);
        }

        let current_height = grid[current];
        for (neighbor_point, neighbor_height) in
            grid.neighbors_with_values(current, Neighbors::Four)
        {
            if *neighbor_height <= current_height + 1 {
                frontier.push_back((neighbor_point, len + 1));
            }
        }
    }
    None
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let mut grid = Grid::from_str(input, "\n", None, |c| {
        let c = c.chars().next().unwrap();
        match c {
            'a'..='z' => (c as u8) - ('a' as u8),
            'S' => START_SENTINEL,
            'E' => END_SENTINEL,
            _ => panic!("Unknown grid char {:?}", c),
        }
    });
    let mut start = None;
    let mut end = None;
    for (p, v) in grid.iter_with_points() {
        if *v == START_SENTINEL {
            start = Some(p);
        } else if *v == END_SENTINEL {
            end = Some(p);
        }
    }
    let start = start.expect("No start position marked!");
    let end = end.expect("No end position marked!");
    grid[start] = 0;
    grid[end] = 25;

    let len = do_bfs(start, end, &grid).unwrap();

    println!("Part 1: {}", len);

    let shortest_path = grid
        .iter_with_points()
        .filter_map(|(p, h)| if *h == 0 { Some(p) } else { None })
        .filter_map(|candidate_start| do_bfs(candidate_start, end, &grid))
        .min()
        .unwrap();
    println!("Part 2: {}", shortest_path);
}
