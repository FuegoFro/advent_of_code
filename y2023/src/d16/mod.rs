use itertools::Itertools;
use std::collections::HashSet;

use serde::Deserialize;

use util::grid::Grid;
use util::point2::{DeltaU, PointU};

#[derive(Eq, PartialEq, Deserialize)]
enum Cell {
    #[serde(rename = ".")]
    Empty,
    #[serde(rename = "/")]
    MirrorForward,
    #[serde(rename = r"\")]
    MirrorBackward,
    #[serde(rename = "|")]
    SplitVertical,
    #[serde(rename = "-")]
    SplitHorizontal,
}

impl Cell {
    fn new_dirs(&self, incoming_dir: Direction) -> Vec<Direction> {
        match self {
            Cell::Empty => vec![incoming_dir],
            Cell::MirrorForward => {
                vec![match incoming_dir {
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Right,
                }]
            }
            Cell::MirrorBackward => {
                vec![match incoming_dir {
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Left,
                }]
            }
            Cell::SplitVertical => match incoming_dir {
                Direction::Right | Direction::Left => vec![Direction::Up, Direction::Down],
                Direction::Up | Direction::Down => vec![incoming_dir],
            },
            Cell::SplitHorizontal => match incoming_dir {
                Direction::Right | Direction::Left => vec![incoming_dir],
                Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
            },
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn to_delta(&self) -> DeltaU {
        match self {
            Direction::Right => DeltaU::RIGHT,
            Direction::Down => DeltaU::DOWN,
            Direction::Left => DeltaU::LEFT,
            Direction::Up => DeltaU::UP,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Beam {
    pos: PointU,
    dir: Direction,
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let grid = Grid::<Cell>::from_serde_chars(input);

    let initial_beam = Beam {
        pos: PointU::ORIGIN,
        dir: Direction::Right,
    };

    let p1 = count_energized(&grid, initial_beam);

    println!("Part 1: {}", p1);

    let p2 = (0..grid.width())
        .map(|x| Beam {
            pos: PointU::new(x, 0),
            dir: Direction::Down,
        })
        .chain((0..grid.width()).map(|x| Beam {
            pos: PointU::new(x, grid.height() - 1),
            dir: Direction::Up,
        }))
        .chain((0..grid.height()).map(|y| Beam {
            pos: PointU::new(0, y),
            dir: Direction::Right,
        }))
        .chain((0..grid.height()).map(|y| Beam {
            pos: PointU::new(grid.width() - 1, y),
            dir: Direction::Left,
        }))
        .map(|initial_beam| count_energized(&grid, initial_beam))
        .max()
        .unwrap();
    println!("Part 2: {}", p2);
}

fn count_energized(grid: &Grid<Cell>, initial_beam: Beam) -> usize {
    let mut frontier = vec![initial_beam];
    let mut seen = HashSet::new();
    while let Some(current) = frontier.pop() {
        if seen.contains(&current) {
            continue;
        }
        seen.insert(current.clone());
        for new_dir in grid[current.pos].new_dirs(current.dir) {
            if let Some(new_pos) = current.pos.checked_add(&new_dir.to_delta()) {
                if grid.get(new_pos).is_some() {
                    frontier.push(Beam {
                        pos: new_pos,
                        dir: new_dir,
                    });
                }
            }
        }
    }

    seen.iter().map(|b| b.pos).unique().count()
}
