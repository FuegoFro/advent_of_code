use crate::util::grid::Grid;
use crate::util::point2::{Delta, PointU};
use itertools::Itertools;
use recap::Recap;
use std::fmt::{Debug, Formatter, Write};

enum Rotate {
    Right,
    Left,
    Noop,
}

impl Rotate {
    fn from_str(s: &str) -> Self {
        match s {
            "R" => Self::Right,
            "L" => Self::Left,
            _ => panic!("Unknown direction str {:?}", s),
        }
    }
}

fn parse_instructions(mut s: &str) -> Vec<(usize, Rotate)> {
    let mut instructions = Vec::new();
    loop {
        if let Some(pos) = s.chars().position(|c| c == 'R' || c == 'L') {
            let (count_raw, new_s) = s.split_at(pos);
            let (rotate_raw, new_s) = new_s.split_at(1);
            s = new_s;
            instructions.push((count_raw.parse().unwrap(), Rotate::from_str(rotate_raw)));
        } else {
            instructions.push((s.parse().unwrap(), Rotate::Noop));
            return instructions;
        }
    }
}

#[derive(Eq, PartialEq)]
enum Cell {
    Gone,
    Empty,
    Wall,
    Character,
}

impl Cell {
    fn from_str(s: &str) -> Self {
        match s {
            " " => Self::Gone,
            "." => Self::Empty,
            "#" => Self::Wall,
            _ => panic!("Unknown cell {:?}", s),
        }
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Cell::Gone => ' ',
            Cell::Empty => '.',
            Cell::Wall => '#',
            Cell::Character => 'X',
        })
    }
}

fn find_next_available(grid: &Grid<Cell>, current: &PointU, direction: &Delta) -> Option<PointU> {
    let width = grid.width() as i32;
    let height = grid.height() as i32;
    let mut current = current.as_signed();
    loop {
        current += direction;
        current.x = current.x.rem_euclid(width);
        current.y = current.y.rem_euclid(height);
        let current_signed = current.as_unsigned();
        match grid[current_signed] {
            Cell::Gone => continue,
            Cell::Empty => return Some(current_signed),
            Cell::Wall => return None,
            Cell::Character => unreachable!(),
        }
    }
}

fn calculate_password(position: PointU, direction: Delta) -> usize {
    let direction_score = match direction {
        Delta::RIGHT => 0,
        Delta::DOWN => 1,
        Delta::LEFT => 2,
        Delta::UP => 3,
        _ => panic!("Invalid direction"),
    };
    1000 * (position.y + 1) + 4 * (position.x + 1) + direction_score
}

/*
rotation is "up" for the sides, "away" for the top, and "close" for the bottom
0 == top
1 == front
2 == right
3 == back
4 == left
5 == bottom

----
A
BCDE
F
----

Face {
    orig_location: PointU,
    orig_rotation: usize,
    data: Grid<Cell>,
    connections: Vec<(&Face, Direction)>,
}

----
 EF
 D
BC
A
----
F = 0r0
E = 4r90
D = 1r90
C = 2r90
B = 5r0
A = 3r180
----

----
  A
BCD
  EF
----
A = 0r0
D = 1r0
C = 4r0
B = 3r0
E = 5r0
F = 2r90
----

0up -> 3up (r)
0right -> 2up (r)
0down -> 1up (r)
0left -> 4up (r)

1right -> 2left (r)
1down -> 5up (r)
1left -> 4right (r)


*/

// fn grid_point_rotated()

struct Face<'a> {
    orig_start: PointU,
    orig_rotation: usize,
    data: Grid<Cell>,
    /// up, right, down, left
    connections: Vec<(&'a Face<'a>, usize)>,
}

impl<'a> Face<'a> {
    fn new(source: Grid<Cell>, start: PointU, end: PointU, rotation: usize) -> Self {
        todo!()
    }
}

// Extract from original data
// Connect

pub fn main() {
    // let input = include_str!("example_input.txt")
    //     .trim_end()
    //     .replace('\r', "");
    let input = include_str!("actual_input.txt")
        .trim_end()
        .replace('\r', "");

    let (raw_grid, raw_instructions) = input.split_once("\n\n").unwrap();
    let max_line_len = raw_grid.split('\n').map(|l| l.len()).max().unwrap();
    let padded_raw_grid = raw_grid
        .split('\n')
        .map(|l| {
            let mut l = l.to_string();
            while l.len() < max_line_len {
                l.push(' ');
            }
            l
        })
        .join("\n");
    let mut grid = Grid::from_str(padded_raw_grid, "\n", None, Cell::from_str);
    let instructions = parse_instructions(raw_instructions);

    // dbg!(&grid);

    let mut direction = Delta::RIGHT;
    let mut position = grid
        .iter_with_points()
        .filter(|(_, c)| **c == Cell::Empty)
        .map(|(p, _)| p)
        .next()
        .unwrap();

    for (count, rotate) in instructions {
        for _ in 0..count {
            if let Some(next) = find_next_available(&grid, &position, &direction) {
                position = next;
                grid[position] = Cell::Character;
                // dbg!(&grid);
                grid[position] = Cell::Empty;
            } else {
                break;
            }
        }
        direction = match rotate {
            Rotate::Right => direction.rotate_about_origin_deg(90),
            Rotate::Left => direction.rotate_about_origin_deg(270),
            Rotate::Noop => direction,
        }
    }

    let password = calculate_password(dbg!(position), dbg!(direction));

    println!("Part 1: {}", password);
    // println!("Part 2: {}", 2);
}
