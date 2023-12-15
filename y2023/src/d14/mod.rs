use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Write};

use util::grid::Grid;
use util::point2::{Delta, PointU};

#[derive(Copy, Clone, Hash, Eq, PartialEq, Deserialize)]
enum Cell {
    #[serde(rename = ".")]
    Empty,
    #[serde(rename = "O")]
    Rock,
    #[serde(rename = "#")]
    Wall,
}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Cell::Empty => '.',
            Cell::Rock => 'O',
            Cell::Wall => '#',
        })
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let mut grid = Grid::<Cell>::from_serde_chars(input);

    let mut grid_p1 = grid.clone();
    tilt_north(&mut grid_p1);
    let p1 = calc_load(grid_p1);

    println!("Part 1: {}", p1);

    let mut first_seen_idx = HashMap::new();
    first_seen_idx.insert(grid.clone(), 0);
    let mut idx = 0;
    loop {
        idx += 1;
        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);
        if first_seen_idx.contains_key(&grid) {
            break;
        }
        first_seen_idx.insert(grid.clone(), idx);
        // dbg!(&grid);
    }
    // println!("{} {}", first_seen_idx.get(&grid).unwrap(), idx);
    let offset = first_seen_idx.get(&grid).unwrap();
    let diff = offset - idx;
    let target_idx = ((1000000000 - offset) % diff) + offset;
    let final_grid = first_seen_idx
        .into_iter()
        .filter(|(_, i)| *i == target_idx)
        .map(|(g, _)| g)
        .next()
        .unwrap();
    let p2 = calc_load(final_grid);

    println!("Part 2: {}", p2);
}

fn delta_iter(start: Delta<isize>, end: Delta<isize>) -> impl Iterator<Item = Delta<isize>> {
    struct PointIter {
        next: Delta<isize>,
        delta: Delta<isize>,
        end: Delta<isize>,
    }

    impl Iterator for PointIter {
        type Item = Delta<isize>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.next == self.end {
                None
            } else {
                let result = Some(self.next);
                self.next += self.delta;
                result
            }
        }
    }

    PointIter {
        next: start,
        delta: (end - start).unit(),
        end,
    }
}

fn tilt_north(grid: &mut Grid<Cell>) {
    tilt(
        grid,
        Delta::new(0, 0),
        Delta::new(grid.width() as isize, 0),
        Delta::new(0, 0),
        Delta::new(0, grid.height() as isize),
    );
}

fn tilt_west(grid: &mut Grid<Cell>) {
    tilt(
        grid,
        Delta::new(0, 0),
        Delta::new(0, grid.height() as isize),
        Delta::new(0, 0),
        Delta::new(grid.width() as isize, 0),
    );
}

fn tilt_south(grid: &mut Grid<Cell>) {
    tilt(
        grid,
        Delta::new(0, 0),
        Delta::new(grid.width() as isize, 0),
        Delta::new(0, grid.height() as isize - 1),
        Delta::new(0, -1),
    );
}

fn tilt_east(grid: &mut Grid<Cell>) {
    tilt(
        grid,
        Delta::new(0, 0),
        Delta::new(0, grid.height() as isize),
        Delta::new(grid.width() as isize - 1, 0),
        Delta::new(-1, 0),
    );
}

fn tilt(
    grid: &mut Grid<Cell>,
    outer_start: Delta<isize>,
    outer_end: Delta<isize>,
    inner_start: Delta<isize>,
    inner_end: Delta<isize>,
) {
    let inner_direction = (inner_end - inner_start).unit();

    for outer in delta_iter(outer_start, outer_end) {
        let mut first_available = PointU::ORIGIN + outer + inner_start;
        for inner in delta_iter(inner_start, inner_end) {
            let current = PointU::ORIGIN + outer + inner;
            match grid[current] {
                Cell::Empty => {}
                Cell::Rock => {
                    grid[current] = Cell::Empty;
                    grid[first_available] = Cell::Rock;
                    if let Some(next_space) = first_available.checked_add(&inner_direction) {
                        first_available = next_space;
                    }
                }
                Cell::Wall => {
                    if let Some(next_space) = current.checked_add(&inner_direction) {
                        first_available = next_space;
                    }
                }
            }
        }
    }
}

fn calc_load(grid: Grid<Cell>) -> usize {
    grid.iter_with_points()
        .filter(|(_, c)| **c == Cell::Rock)
        .map(|(p, _)| grid.height() - p.y)
        .sum::<usize>()
}
