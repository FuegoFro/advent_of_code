use crate::util::p_u32c;
use crate::util::point::Point;
use itertools::Itertools;
use std::collections::HashMap;

const OFFSETS: [Point; 4] = [
    Point::new(1, 0),
    Point::new(-1, 0),
    Point::new(0, 1),
    Point::new(0, -1),
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

/// Assumes that basins are fully delimited by 9's
fn get_basin_sizes_product(nums: &Vec<Vec<u32>>) -> i32 {
    let mut points_in_basin = HashMap::new();
    for y in 0..nums.len() {
        for x in 0..nums[y].len() {
            let start_point = Point::new(x as i32, y as i32);
            let start_value = nums[y][x];
            if start_value == 9 {
                continue;
            }
            let mut current_point = start_point;
            let mut current_value = start_value;
            loop {
                let result = get_neighbors(nums, current_point)
                    .into_iter()
                    .filter(|(_, v)| *v < current_value)
                    .reduce(|a, b| if a.1 < b.1 { a } else { b });
                match result {
                    Some((next_point, next_value)) => {
                        if next_point == current_point {
                            break;
                        }
                        current_point = next_point;
                        current_value = next_value;
                    }
                    None => break,
                }
            }
            *points_in_basin.entry(current_point).or_insert(0) += 1;
        }
    }

    points_in_basin
        .into_values()
        .sorted()
        .rev()
        .take(3)
        .product()
}

pub fn main() {
    let input = include_str!("example_input.txt").trim().replace("\r", "");
    // let input = include_str!("actual_input.txt").trim().replace("\r", "");

    let nums = input
        .split("\n")
        .map(|l| l.chars().map(p_u32c).collect_vec())
        .collect_vec();

    let mut low_points = vec![];
    for y in 0..nums.len() {
        for x in 0..nums[y].len() {
            let current = nums[y][x];
            if get_neighbors(&nums, Point::new(x as i32, y as i32))
                .iter()
                .all(|(_, neighbor)| *neighbor > current)
            {
                low_points.push(current);
            }
        }
    }
    let total_risk: u32 = low_points.into_iter().map(|v| v + 1).sum();
    println!("Part 1: {}", total_risk);

    println!("Part 2: {}", get_basin_sizes_product(&nums));
}
