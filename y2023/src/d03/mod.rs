use itertools::Itertools;
use std::collections::HashSet;
use util::grid::{Grid, Neighbors};
use util::point2::{Delta, PointU};

#[derive(Debug, Eq, PartialEq)]
enum Cell {
    Digit(u32),
    Symbol(char),
    Empty,
}

impl Cell {
    fn from_char(s: &str) -> Self {
        let c = s.chars().next().unwrap();
        if let Some(digit) = c.to_digit(10) {
            Cell::Digit(digit)
        } else if c == '.' {
            Cell::Empty
        } else {
            Cell::Symbol(c)
        }
    }

    fn is_symbol(&self) -> bool {
        matches!(self, Cell::Symbol(_))
    }

    fn is_digit(&self) -> bool {
        matches!(self, Cell::Digit(_))
    }
}

struct NumInfo {
    value: u32,
    start: PointU,
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let grid: Grid<Cell> = Grid::from_str(input, "\n", None, Cell::from_char);

    let mut total = 0;
    let mut num_info = None;
    for (point, cell) in grid.iter_with_points() {
        if point.x == 0 && point.y > 0 {
            process_number(
                &grid,
                &mut total,
                PointU::new(grid.width(), point.y - 1),
                num_info.take(),
            );
        }
        match cell {
            Cell::Digit(digit) => {
                let info = num_info.get_or_insert(NumInfo {
                    value: 0,
                    start: point,
                });
                info.value = info.value * 10 + digit;
            }
            _ => {
                process_number(&grid, &mut total, point, num_info.take());
            }
        }
    }

    println!("Part 1: {}", total);

    let mut ratios = 0;
    for (point, cell) in grid.iter_with_points() {
        if *cell == Cell::Symbol('*') {
            let neighbor_digits = grid
                .neighbors_with_values(point, Neighbors::Eight)
                .filter(|(_, cell)| cell.is_digit())
                .map(|(p, _)| p)
                .collect_vec();

            let mut neighbor_numbers = vec![];
            let mut seen_points = HashSet::new();
            for point in neighbor_digits {
                if seen_points.contains(&point) {
                    continue;
                }
                neighbor_numbers.push(extract_number(&grid, point, &mut seen_points))
            }

            if neighbor_numbers.len() == 2 {
                ratios += neighbor_numbers.into_iter().product::<u32>()
            }
        }
    }

    println!("Part 2: {}", ratios);
}

fn process_number(grid: &Grid<Cell>, total: &mut u32, end: PointU, info: Option<NumInfo>) {
    if let Some(info) = info {
        let mut current_point = info.start;
        while current_point != end {
            if grid
                .neighbors_with_values(current_point, Neighbors::Eight)
                .any(|(_, n)| n.is_symbol())
            {
                *total += info.value;
                break;
            }
            current_point += Delta::RIGHT
        }
    }
}

fn extract_number(grid: &Grid<Cell>, point: PointU, seen_points: &mut HashSet<PointU>) -> u32 {
    let mut start = point;
    while start.x > 0 && grid[start + Delta::LEFT].is_digit() {
        start.x -= 1
    }
    let mut end = point + Delta::RIGHT;
    while end.x < grid.width() && grid[end].is_digit() {
        end.x += 1
    }
    let mut num = 0;
    while start != end {
        seen_points.insert(start);
        let Cell::Digit(digit) = grid[start] else {
            unreachable!()
        };
        num = num * 10 + digit;
        start.x += 1;
    }
    num
}
