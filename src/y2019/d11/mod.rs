use crate::util::point::{get_bounding_box, Point};
use crate::y2019::computer::{Computer, ComputerExitStatus, Word};
use std::collections::HashMap;

struct Robot {
    position: Point,
    direction: Point,
    hull: HashMap<Point, Word>,
}

impl Robot {
    fn new() -> Self {
        Robot {
            position: Point::new(0, 0),
            direction: Point::UP,
            hull: Default::default(),
        }
    }

    fn get_color_at_current_location(&self) -> Word {
        *self.hull.get(&self.position).unwrap_or(&0)
    }

    fn paint_and_move(&mut self, color: Word, rotation: Word) {
        self.hull.insert(self.position, color);
        let rotation_degs = match rotation {
            0 => 90,
            1 => 270,
            _ => panic!("Unknown rotation value: {}", rotation),
        };
        self.direction = self.direction.rotate_about_origin_deg(rotation_degs);
        self.position += self.direction;
    }
}

pub fn main() {
    let input = include_str!("actual_input.txt").trim();

    let robot = run_painter(input, 0);
    println!("{}", robot.hull.len());

    let robot = run_painter(input, 1);
    let (origin, opposite_corner) = get_bounding_box(robot.hull.iter().map(|(p, _)| p));
    let size = opposite_corner - origin + Point::new(1, 1);

    let mut grid = vec![vec![false; size.x as usize]; size.y as usize];
    for (point, color) in robot.hull.iter() {
        let point = point - origin;
        grid[point.y as usize][point.x as usize] = match color {
            0 => false,
            1 => true,
            _ => panic!("Unexpected color: {}", color),
        }
    }
    for row in grid.iter().rev() {
        for val in row.iter() {
            if *val {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn run_painter(input: &str, initial_square: i64) -> Robot {
    let mut computer = Computer::from_packed(input);
    let mut robot = Robot::new();

    computer.send_as_input(initial_square);
    while computer.run() == ComputerExitStatus::WaitingForInput {
        let mut outputs = computer.drain_outputs().into_iter();
        while let Some(color) = outputs.next() {
            let rotation = outputs.next().unwrap();
            robot.paint_and_move(color, rotation);
        }
        computer.send_as_input(robot.get_color_at_current_location());
    }

    robot
}
