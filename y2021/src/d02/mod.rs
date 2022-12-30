use itertools::Itertools;
use util::p_i32;
use util::point::Point;

#[derive(Clone)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl Direction {
    fn from_str(s: &str) -> Self {
        match s {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => panic!("Unknown direction '{}'", s),
        }
    }
}

fn parse_line(line: &str) -> (Direction, i32) {
    let (first, second) = line.split_once(" ").unwrap();
    (Direction::from_str(first), p_i32(second))
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let directions = input.split("\n").map(parse_line).collect_vec();

    // P1
    let mut current_pos = Point::new(0, 0);
    for (direction, distance) in directions.clone() {
        let vector = match direction {
            Direction::Forward => Point::new(distance, 0),
            Direction::Down => Point::new(0, distance),
            Direction::Up => Point::new(0, -distance),
        };
        current_pos += vector;
    }

    println!("Part 1: {}", current_pos.x * current_pos.y);

    // P2
    let mut current_pos = Point::new(0, 0);
    let mut heading = 0;
    for (direction, amount) in directions.clone() {
        match direction {
            Direction::Down => heading += amount,
            Direction::Up => heading -= amount,
            Direction::Forward => current_pos += Point::new(amount, heading * amount),
        }
    }

    println!("Part 2: {}", current_pos.x * current_pos.y);
}
