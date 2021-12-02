use crate::util::point::{get_bounding_box, Point};
use crate::y2019::computer::{Computer, Word};
use itertools::Itertools;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::TryInto;
use std::io::{BufRead, Write};
use std::iter::FromIterator;
use std::{io, iter};

#[derive(Debug, FromPrimitive, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Wall = 0,
    Empty = 1,
    Oxygen = 2,
}

#[derive(Debug, Copy, Clone)]
enum Move {
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
}

impl Move {
    fn direction(&self) -> &Point {
        match self {
            Move::Up => &Point::UP,
            Move::Down => &Point::DOWN,
            Move::Left => &Point::LEFT,
            Move::Right => &Point::RIGHT,
        }
    }
}

struct Robot {
    position: Point,
    map: HashMap<Point, Tile>,
}

impl Robot {
    fn new() -> Self {
        Robot {
            position: Point::new(0, 0),
            map: iter::once((Point::new(0, 0), Tile::Empty)).collect(),
        }
    }

    fn render_map(&self) {
        let (l, u) = get_bounding_box(self.map.keys().collect_vec());
        for y in (l.y..=u.y).rev() {
            for x in l.x..=u.x {
                let current = Point::new(x, y);
                let char = if current == self.position {
                    "D"
                } else if current == Point::ORIGIN {
                    "@"
                } else {
                    self.map
                        .get(&current)
                        .map(|tile| match tile {
                            Tile::Wall => "#",
                            Tile::Empty => ".",
                            Tile::Oxygen => "O",
                        })
                        .unwrap_or("?")
                };
                print!("{}", char);
            }
            println!();
        }
        println!("D = {:?}", self.position);
        if self.map[&self.position] == Tile::Oxygen {
            println!("On oxygen!!!")
        }
    }
}

#[allow(dead_code)]
fn get_move() -> Move {
    loop {
        print!("Enter a direction [wasd]: ");
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let entered_line = stdin.lock().lines().next().unwrap().unwrap();
        return match entered_line.to_ascii_lowercase().as_ref() {
            "w" => Move::Up,
            "a" => Move::Left,
            "s" => Move::Down,
            "d" => Move::Right,
            _ => {
                println!("Unknown move: {}", entered_line);
                continue;
            }
        };
    }
}

fn do_move(computer: &mut Computer, robot: &mut Robot, next_move: Move) -> (Point, Tile) {
    // println!("Moving: {:?}", next_move);
    computer.send_as_input(next_move as Word);
    robot.position += next_move.direction();
    computer.run().assert_waiting_for_input();
    let [output]: [Word; 1] = computer.drain_outputs().as_slice().try_into().unwrap();
    let tile = Tile::from_i64(output).unwrap();
    let tile_pos = robot.position;
    robot.map.insert(tile_pos, tile);
    if tile == Tile::Wall {
        robot.position -= next_move.direction();
    }
    // println!("Got {:?} at {:?}", tile, tile_pos);
    (tile_pos, tile)
}

pub fn main() {
    let input = include_str!("actual_input.txt").trim();
    let mut robot = Robot::new();
    let mut computer = Computer::from_packed(input);

    // First go north to find some wall
    let mut target = Point::ORIGIN;
    let mut tile = Tile::Empty;
    while tile == Tile::Empty {
        println!("In initial loop");
        let (new_target, new_tile) = do_move(&mut computer, &mut robot, Move::Up);
        target = new_target;
        tile = new_tile;
    }
    if tile == Tile::Oxygen {
        println!("Initial run: {:?}", robot.position);
        return;
    }

    // Then loop exploring the edges.
    let initial_point = robot.position;
    let mut has_moved = false;
    // let mut has_moved = true;
    while !has_moved || robot.position != initial_point {
        // println!("Exploring: robot={:?} target={:?}", robot.position, target);
        let (move_a, move_b) = match target - robot.position {
            Point::UP => (Move::Right, Move::Up),
            Point::RIGHT => (Move::Down, Move::Right),
            Point::DOWN => (Move::Left, Move::Down),
            Point::LEFT => (Move::Up, Move::Left),
            _ => panic!(
                "Target not next to robot! target={:?}, robot={:?}",
                target, robot.position
            ),
        };
        let (new_target, new_tile) = do_move(&mut computer, &mut robot, move_a);
        match new_tile {
            Tile::Wall => {
                target = new_target;
            }
            Tile::Empty | Tile::Oxygen => {
                let (new_target, new_tile) = do_move(&mut computer, &mut robot, move_b);
                match new_tile {
                    Tile::Wall => {
                        target = new_target;
                    }
                    Tile::Empty | Tile::Oxygen => {}
                }
            }
        }
        if robot.position != initial_point {
            has_moved = true;
        }
    }

    // Do search from oxygen to origin
    let [oxygen_location]: [Point; 1] = robot
        .map
        .iter()
        .filter(|(_, t)| **t == Tile::Oxygen)
        .map(|(p, _)| *p)
        .collect_vec()
        .as_slice()
        .try_into()
        .unwrap();

    let mut frontier = VecDeque::from_iter(iter::once((oxygen_location, 0)));
    let mut seen = HashSet::new();
    let mut max_dist = 0;
    while let Some((next_point, dist)) = frontier.pop_front() {
        if seen.contains(&next_point) {
            continue;
        }
        seen.insert(next_point);
        max_dist = max(max_dist, dist);

        if next_point == Point::ORIGIN {
            println!("Oxygen dist from origin: {}", dist);
        }

        match robot.map.get(&next_point) {
            Some(Tile::Empty) | Some(Tile::Oxygen) => {
                for delta in [Point::UP, Point::DOWN, Point::LEFT, Point::RIGHT].iter() {
                    frontier.push_back((next_point + delta, dist + 1));
                }
            }
            Some(Tile::Wall) => continue,
            None => {
                robot.render_map();
                panic!("Don't know what tile is at: {:?}", next_point)
            }
        }
    }
    println!("Minutes to refill ship {}", max_dist);
}
