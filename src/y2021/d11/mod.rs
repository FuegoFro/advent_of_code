use crate::util::p_u32c;
use crate::util::point::Point;
use itertools::Itertools;
use std::collections::HashSet;

const OFFSETS: [Point; 8] = [
    Point::new(-1, -1),
    Point::new(0, -1),
    Point::new(1, -1),
    Point::new(-1, 0),
    Point::new(1, 0),
    Point::new(-1, 1),
    Point::new(0, 1),
    Point::new(1, 1),
];

fn get_neighbors(nums: &Vec<Vec<u32>>, p: Point) -> Vec<(Point, u32)> {
    OFFSETS
        .iter()
        .map(|o| o + p)
        .filter(|p| p.x >= 0 && p.y >= 0)
        .flat_map(|p| {
            nums.get(p.y as usize)
                .and_then(|row| row.get(p.x as usize))
                .and_then(|value| Some((p, *value)))
        })
        .collect_vec()
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<u32>>) {
    println!(
        "{}",
        grid.iter()
            .map(|r| r
                .iter()
                .map(|v| if *v > 9 {
                    "x".to_string()
                } else {
                    v.to_string()
                })
                .join(""))
            .join("\n")
    );
}

fn run_cycle(grid: &mut Vec<Vec<u32>>) -> usize {
    // Initial increment
    grid.iter_mut().flat_map(|row| row).for_each(|v| *v += 1);

    let mut to_flash_frontier = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, v)| {
                // println!("({}, {}) = {}", x, y, v);
                if *v > 9 {
                    Some(Point::new(x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect_vec();
    let mut flashed = HashSet::new();
    while let Some(to_flash) = to_flash_frontier.pop() {
        if flashed.contains(&to_flash) {
            continue;
        }
        flashed.insert(to_flash);
        for (neighbor, _) in get_neighbors(grid, to_flash) {
            grid[neighbor.y as usize][neighbor.x as usize] += 1;
            if grid[neighbor.y as usize][neighbor.x as usize] > 9 {
                to_flash_frontier.push(neighbor);
            }
        }
        // print_grid(&grid);
    }
    // Reset flashed to 0
    let total = flashed.len();
    for p in flashed {
        grid[p.y as usize][p.x as usize] = 0
    }

    total
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace("\r", "");

    let initial_grid = input
        .split("\n")
        .map(|line| line.chars().map(p_u32c).collect_vec())
        .collect_vec();

    let mut part1 = initial_grid.clone();
    let total: usize = (0..100).map(|_| run_cycle(&mut part1)).sum();
    println!("Part 1: {}", total);

    let mut part2 = initial_grid.clone();
    let mut steps = 1;
    while run_cycle(&mut part2) != 100 {
        steps += 1;
    }
    println!("Part 2: {}", steps);
}
