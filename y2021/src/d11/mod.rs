use std::collections::HashSet;

use itertools::Itertools;

use util::grid::{Grid, Neighbors};
use util::p_u32;

fn run_cycle(grid: &mut Grid<u32>) -> usize {
    // Initial increment
    grid.iter_mut().for_each(|v| *v += 1);

    let mut to_flash_frontier = grid
        .iter_with_points()
        .filter_map(move |(p, v)| if *v > 9 { Some(p) } else { None })
        .collect_vec();
    let mut flashed = HashSet::new();
    while let Some(to_flash) = to_flash_frontier.pop() {
        if flashed.contains(&to_flash) {
            continue;
        }
        flashed.insert(to_flash);
        for neighbor in grid.neighbors(to_flash, Neighbors::Eight) {
            grid[neighbor] += 1;
            if grid[neighbor] > 9 {
                to_flash_frontier.push(neighbor);
            }
        }
    }

    // Reset flashed to 0, get total flashed. The `sum` is definitely a bit of a hack.
    flashed
        .into_iter()
        .map(|p| {
            grid[p] = 0;
            1
        })
        .sum()
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let initial_grid = Grid::from_str(input, "\n", None, p_u32);

    let mut part1 = initial_grid.clone();
    let total: usize = (0..100).map(|_| run_cycle(&mut part1)).sum();
    println!("Part 1: {}", total);

    let mut part2 = initial_grid;
    let mut steps = 1;
    while run_cycle(&mut part2) != 100 {
        steps += 1;
    }
    println!("Part 2: {}", steps);
}
