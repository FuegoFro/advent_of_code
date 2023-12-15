use itertools::Itertools;
use std::cmp::min;
use util::grid::Grid;
use util::point2::PointU;

#[derive(Debug, Eq, PartialEq)]
enum Cell {
    Ash,
    Rock,
}

impl Cell {
    fn from_str(s: &str) -> Self {
        match s {
            "." => Self::Ash,
            "#" => Self::Rock,
            _ => panic!("Unknown str {}", s),
        }
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let grids = input
        .split("\n\n")
        .map(|pattern| Grid::<Cell>::from_str(pattern, "\n", None, Cell::from_str))
        .collect_vec();

    let p1 = grids.iter().map(|g| get_score(g, 0)).sum::<usize>();
    println!("Part 1: {}", p1);

    let p2 = grids.iter().map(|g| get_score(g, 1)).sum::<usize>();
    println!("Part 2: {}", p2);
}

fn get_score(grid: &Grid<Cell>, num_errors: u32) -> usize {
    if let Some(col_idx) = get_mirror_col(grid, num_errors) {
        col_idx
    } else if let Some(row_idx) = get_mirror_row(grid, num_errors) {
        100 * row_idx
    } else {
        panic!("Couldn't find mirror col or row for:\n{:?}", grid)
    }
}

fn get_mirror_col(grid: &Grid<Cell>, num_errors: u32) -> Option<usize> {
    for x_part_2 in 1..grid.width() {
        // Check if mirrors
        let x_part_1 = x_part_2 - 1;
        let len = min(x_part_1 + 1, grid.width() - x_part_2);
        if (0..len)
            .map(|offset| {
                (0..grid.height())
                    .map(|y| {
                        let matches = grid[PointU::new(x_part_1 - offset, y)]
                            == grid[PointU::new(x_part_2 + offset, y)];
                        !matches as u32
                    })
                    .sum::<u32>()
            })
            .sum::<u32>()
            == num_errors
        {
            return Some(x_part_2);
        }
    }
    None
}

fn get_mirror_row(grid: &Grid<Cell>, num_errors: u32) -> Option<usize> {
    for y_part_2 in 1..grid.height() {
        // Check if mirrors
        let y_part_1 = y_part_2 - 1;
        let len = min(y_part_1 + 1, grid.height() - y_part_2);
        if (0..len)
            .map(|offset| {
                (0..grid.width())
                    .map(|x| {
                        let matches = grid[PointU::new(x, y_part_1 - offset)]
                            == grid[PointU::new(x, y_part_2 + offset)];
                        !matches as u32
                    })
                    .sum::<u32>()
            })
            .sum::<u32>()
            == num_errors
        {
            return Some(y_part_2);
        }
    }
    None
}
