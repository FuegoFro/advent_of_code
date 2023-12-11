use std::collections::HashSet;
use std::fmt::{Debug, Formatter, Write};

use itertools::Itertools;

use util::grid::{Grid, Neighbors};
use util::point2::{Delta, PointU};

#[derive(Debug, Eq, PartialEq)]
enum Cell {
    NS,
    EW,
    NW,
    NE,
    SW,
    SE,
    Start,
    Empty,
}

impl Cell {
    fn from_str(s: &str) -> Self {
        match s {
            "|" => Cell::NS,
            "-" => Cell::EW,
            "J" => Cell::NW,
            "L" => Cell::NE,
            "7" => Cell::SW,
            "F" => Cell::SE,
            "S" => Cell::Start,
            "." => Cell::Empty,
            _ => panic!("Unknown cell {}", s),
        }
    }

    fn can_connect(&self, my_point: PointU, other: &Self, other_point: PointU) -> bool {
        let result = self.can_connect_inner(my_point, other_point)
            && other.can_connect_inner(other_point, my_point);
        // eprintln!(
        //     "Checking ({:?}, {:?}) -> ({:?}, {:?}) = {}",
        //     self, my_point, other, other_point, result
        // );
        result
    }

    fn can_connect_inner(&self, my_point: PointU, other_point: PointU) -> bool {
        match other_point - my_point {
            // Connect on the north side
            Delta::UP => [Cell::NW, Cell::NS, Cell::NE, Cell::Start].contains(self),
            // Connect on the south side
            Delta::DOWN => [Cell::SW, Cell::NS, Cell::SE, Cell::Start].contains(self),
            // Connect on the west side
            Delta::LEFT => [Cell::SW, Cell::EW, Cell::NW, Cell::Start].contains(self),
            // Connect on the east side
            Delta::RIGHT => [Cell::SE, Cell::EW, Cell::NE, Cell::Start].contains(self),
            _ => panic!("Invalid jump from {:?} to {:?}", other_point, my_point),
        }
    }
}

#[derive(Default)]
enum Cell2 {
    Wall,
    Ground,
    #[default]
    Synthetic,
}

impl Debug for Cell2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Cell2::Wall => '#',
            Cell2::Ground => 'x',
            Cell2::Synthetic => '.',
        })
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let grid: Grid<Cell> = Grid::from_str(input, "\n", None, Cell::from_str);
    let start = grid
        .iter_with_points()
        .find(|(_, c)| **c == Cell::Start)
        .unwrap()
        .0;

    let mut path_len = 0;
    let mut prev = start;
    let mut pos = start;
    let mut path_points = vec![];
    while pos != start || path_len == 0 {
        path_points.push(pos);
        let current = &grid[pos];
        let next = grid
            .neighbors_with_values(pos, Neighbors::Four)
            .find(|(next_p, next_c)| *next_p != prev && current.can_connect(pos, next_c, *next_p))
            .unwrap()
            .0;
        prev = pos;
        pos = next;
        path_len += 1;
    }
    let p1 = path_len / 2;

    println!("Part 1: {}", p1);

    let mut doubled_grid: Grid<Cell2> = Grid::empty(1 + grid.width() * 2, 1 + grid.height() * 2);
    // Default "real" slots to ground
    for p in grid.points() {
        doubled_grid[Delta::DOWN_RIGHT + p * 2] = Cell2::Ground;
    }
    // Insert the walls
    for (a, b) in path_points.iter().chain([&path_points[0]]).tuple_windows() {
        doubled_grid[Delta::DOWN_RIGHT + a * 2] = Cell2::Wall;
        doubled_grid[Delta::DOWN_RIGHT + a * 2 + (b - a)] = Cell2::Wall;
    }
    // dbg!(&doubled_grid);

    let mut frontier = vec![PointU::ORIGIN];

    let mut seen = HashSet::new();
    let mut outside_points = HashSet::new();
    while let Some(current) = frontier.pop() {
        if seen.contains(&current) {
            continue;
        }
        seen.insert(current);
        match doubled_grid[current] {
            Cell2::Wall => continue,
            Cell2::Ground => {
                outside_points.insert(current);
            }
            Cell2::Synthetic => {}
        }
        frontier.extend(doubled_grid.neighbors(current, Neighbors::Four));
    }
    let inside_points = doubled_grid
        .iter_with_points()
        .filter(|(p, c)| matches!(c, Cell2::Ground) && !outside_points.contains(p))
        .count();

    println!("Part 2: {}", inside_points);
}
