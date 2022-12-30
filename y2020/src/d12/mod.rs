use util::point::Point;

enum Operation {
    North,
    East,
    South,
    West,
    Left,
    Right,
    Forward,
}

impl Operation {
    fn from_packed(packed: &str) -> Self {
        match packed {
            "N" => Operation::North,
            "E" => Operation::East,
            "S" => Operation::South,
            "W" => Operation::West,
            "L" => Operation::Left,
            "R" => Operation::Right,
            "F" => Operation::Forward,
            _ => panic!("Unknown operation: {}", packed),
        }
    }
}

enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    fn as_value(&self) -> i32 {
        match self {
            Heading::North => 0,
            Heading::East => 1,
            Heading::South => 2,
            Heading::West => 3,
        }
    }
    fn from_value(value: i32) -> Self {
        match value {
            0 => Heading::North,
            1 => Heading::East,
            2 => Heading::South,
            3 => Heading::West,
            _ => panic!("Unknown heading value: {}", value),
        }
    }
    fn rotate(&self, degrees: u32, clockwise: bool) -> Heading {
        let mut value_delta = degrees as i32 / 90;
        if !clockwise {
            value_delta *= -1;
        };
        // Booooo https://stackoverflow.com/a/31210691/3000133
        let new_value = (((self.as_value() + value_delta) % 4) + 4) % 4;
        Self::from_value(new_value)
    }
}

struct Instruction {
    operation: Operation,
    value: u32,
}

impl Instruction {
    fn from_packed(packed: &str) -> Self {
        let (op, val) = packed.split_at(1);
        Instruction {
            operation: Operation::from_packed(op),
            value: val.parse().unwrap(),
        }
    }
}

struct FerryPt1 {
    position: Point,
    heading: Heading,
}

impl FerryPt1 {
    fn new() -> Self {
        Self {
            position: Point { x: 0, y: 0 },
            heading: Heading::East,
        }
    }

    fn do_instruction(&mut self, instruction: Instruction) {
        let val = instruction.value;
        match instruction.operation {
            Operation::North => self.position += Point::UP * val,
            Operation::East => self.position += Point::RIGHT * val,
            Operation::South => self.position += Point::DOWN * val,
            Operation::West => self.position += Point::LEFT * val,
            Operation::Left => self.heading = self.heading.rotate(val, false),
            Operation::Right => self.heading = self.heading.rotate(val, true),
            Operation::Forward => match self.heading {
                Heading::North => self.position += Point::UP * val,
                Heading::East => self.position += Point::RIGHT * val,
                Heading::South => self.position += Point::DOWN * val,
                Heading::West => self.position += Point::LEFT * val,
            },
        }
    }
}

struct FerryPt2 {
    position: Point,
    waypoint: Point,
}

impl FerryPt2 {
    fn new() -> Self {
        Self {
            position: Point { x: 0, y: 0 },
            waypoint: Point { x: 10, y: 1 },
        }
    }

    fn do_instruction(&mut self, instruction: Instruction) {
        let val = instruction.value;
        match instruction.operation {
            Operation::North => self.waypoint += Point::UP * val,
            Operation::East => self.waypoint += Point::RIGHT * val,
            Operation::South => self.waypoint += Point::DOWN * val,
            Operation::West => self.waypoint += Point::LEFT * val,
            Operation::Left => self.waypoint = self.waypoint.rotate_about_origin_deg(val),
            Operation::Right => self.waypoint = self.waypoint.rotate_about_origin_deg(360 - val),
            Operation::Forward => self.position += self.waypoint * val,
        }
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let mut ferry = FerryPt1::new();
    input
        .split("\n")
        .map(Instruction::from_packed)
        .for_each(|i| ferry.do_instruction(i));
    println!("{} ({:?})", ferry.position.l1_dist(), ferry.position);

    let mut ferry = FerryPt2::new();
    input
        .split("\n")
        .map(Instruction::from_packed)
        .for_each(|i| ferry.do_instruction(i));
    println!("{} ({:?})", ferry.position.l1_dist(), ferry.position);
}
