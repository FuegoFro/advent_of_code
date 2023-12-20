use itertools::Itertools;
use recap::Recap;
use serde::Deserialize;
use std::collections::HashSet;
use util::grid::{Grid, Neighbors};
use util::point2::{DeltaS, DeltaU, PointS, PointU};

#[derive(Deserialize)]
enum Direction {
    #[serde(rename = "U")]
    Up,
    #[serde(rename = "R")]
    Right,
    #[serde(rename = "D")]
    Down,
    #[serde(rename = "L")]
    Left,
}

impl Direction {
    fn to_delta(&self) -> DeltaS {
        match self {
            Direction::Up => DeltaS::UP,
            Direction::Right => DeltaS::RIGHT,
            Direction::Down => DeltaS::DOWN,
            Direction::Left => DeltaS::LEFT,
        }
    }
}

#[derive(Deserialize, Recap)]
#[recap(regex = r"(?P<direction>\w) (?P<count>\d+) \(#(?P<color_hex>\w+)\)")]
struct Instruction {
    direction: Direction,
    count: u32,
    color_hex: String,
}

impl Instruction {
    #[allow(clippy::wrong_self_convention)]
    fn from_color_hex(&self) -> Self {
        let count = u32::from_str_radix(&self.color_hex[0..=4], 16).unwrap();
        let direction = match &self.color_hex[5..=5] {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            bit => panic!("Unknown direction bit {}", bit),
        };
        Self {
            count,
            direction,
            color_hex: String::new(),
        }
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let instructions_p1 = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect_vec();

    let p1 = sparse_grid_size(&instructions_p1);

    println!("Part 1: {}", p1);

    let instructions_p2 = instructions_p1
        .iter()
        .map(|i| i.from_color_hex())
        .collect_vec();

    let p2 = sparse_grid_size(&instructions_p2);

    println!("Part 2: {}", p2);
}

fn sparse_grid_size(instructions: &[Instruction]) -> i64 {
    // First get every stopping point on the x and y axes
    let mut xs = HashSet::new();
    let mut ys = HashSet::new();
    let mut pos = PointS::ORIGIN;
    let mut world_path = vec![pos];
    for i in instructions.iter() {
        let delta = i.direction.to_delta();
        pos += delta * i.count as i32;
        xs.extend([pos.x - 1, pos.x, pos.x + 1]);
        ys.extend([pos.y - 1, pos.y, pos.y + 1]);
        world_path.push(pos);
    }

    let xs = xs.into_iter().sorted().collect_vec();
    let ys = ys.into_iter().sorted().collect_vec();

    let grid_path = world_path
        .iter()
        .map(|p| world_to_grid(*p, &xs, &ys))
        .tuple_windows()
        .flat_map(|(a, b)| a.step_to(&b))
        .collect_vec();

    let grid = Grid::from_points(
        grid_path.iter(),
        // Will include padding on either side so the outside is accessible from the origin
        Some((PointU::ORIGIN, PointU::new(xs.len(), ys.len()))),
    )
    .unwrap();

    let mut frontier = vec![PointU::ORIGIN];
    let mut exterior = HashSet::new();
    while let Some(current) = frontier.pop() {
        if grid[current] == '#' || exterior.contains(&current) {
            continue;
        }
        exterior.insert(current);
        frontier.extend(grid.neighbors(current, Neighbors::Four));
    }

    grid.points()
        .filter(|p| !exterior.contains(p))
        .map(|p| {
            let start = grid_to_world(p, &xs, &ys);
            let end = grid_to_world(p + DeltaU::DOWN_RIGHT, &xs, &ys);
            (end - start).cast::<i64>().unwrap().area()
        })
        .sum::<i64>()
}

fn world_to_grid(world_point: PointS, xs: &[i32], ys: &[i32]) -> PointU {
    PointU::new(
        xs.iter().position(|x| *x == world_point.x).unwrap(),
        ys.iter().position(|y| *y == world_point.y).unwrap(),
    )
}

fn grid_to_world(grid_point: PointU, xs: &[i32], ys: &[i32]) -> PointS {
    PointS::new(xs[grid_point.x], ys[grid_point.y])
}
