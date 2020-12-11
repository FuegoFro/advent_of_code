mod seats {
    use crate::util::point::Point;
    use std::mem;

    const NEIGHBOR_DIRECTIONS: [Point; 8] = [
        Point { x: -1, y: -1 },
        Point { x: -1, y: 0 },
        Point { x: -1, y: 1 },
        Point { x: 0, y: -1 },
        Point { x: 0, y: 1 },
        Point { x: 1, y: -1 },
        Point { x: 1, y: 0 },
        Point { x: 1, y: 1 },
    ];

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub enum SeatState {
        Floor,
        Available,
        Occupied,
    }

    impl SeatState {
        fn from_packed(packed: char) -> Self {
            match packed {
                '.' => SeatState::Floor,
                'L' => SeatState::Available,
                '#' => SeatState::Occupied,
                _ => panic!("Unknown seat state: {}", packed),
            }
        }

        fn occupied_count(&self) -> u8 {
            match self {
                SeatState::Occupied => 1,
                _ => 0,
            }
        }
    }

    pub struct Seats {
        current: Vec<Vec<SeatState>>,
        temp: Vec<Vec<SeatState>>,
    }

    impl Seats {
        pub fn from_packed(packed: &str) -> Self {
            let seats: Vec<_> = packed
                .split("\n")
                .map(|l| l.chars().map(|c| SeatState::from_packed(c)).collect())
                .collect();
            Seats {
                current: seats.clone(),
                temp: seats,
            }
        }

        fn do_step(&mut self) {
            // self.print_current();
            for y in 0..self.temp.len() {
                for x in 0..self.temp[y].len() {
                    let point = Point {
                        x: x as i32,
                        y: y as i32,
                    };
                    self.temp[y][x] = match self.current[y][x] {
                        SeatState::Floor => SeatState::Floor,
                        SeatState::Available => {
                            if self.count_occupied_neighbors(point) == 0 {
                                SeatState::Occupied
                            } else {
                                SeatState::Available
                            }
                        }
                        SeatState::Occupied => {
                            if self.count_occupied_neighbors(point) < 5 {
                                SeatState::Occupied
                            } else {
                                SeatState::Available
                            }
                        }
                    }
                }
            }
            // Make temp the new current
            mem::swap(&mut self.current, &mut self.temp);
        }

        fn get(&self, point: Point) -> Option<&SeatState> {
            if point.x < 0 || point.y < 0 {
                None
            } else {
                self.current
                    .get(point.y as usize)
                    .and_then(|row| row.get(point.x as usize))
            }
        }

        fn count_occupied_neighbors(&self, point: Point) -> u8 {
            NEIGHBOR_DIRECTIONS
                .iter()
                .map(|d| {
                    let mut neighbor = &point + d;
                    while let Some(s) = self.get(neighbor) {
                        match s {
                            SeatState::Floor => neighbor += d,
                            _ => return s.occupied_count(),
                        }
                    }
                    return 0;
                })
                .sum()
        }

        fn print_current(&self) {
            println!("current:");
            for row in self.current.iter() {
                for val in row.iter() {
                    let c = match val {
                        SeatState::Floor => '.',
                        SeatState::Available => 'L',
                        SeatState::Occupied => '#',
                    };
                    print!("{}", c)
                }
                print!("\n")
            }
        }

        pub fn run_until_stable(&mut self) -> u32 {
            self.do_step();
            while self.current != self.temp {
                self.do_step();
            }
            self.current
                .iter()
                .map(|row| row.iter().map(|s| s.occupied_count() as u32).sum::<u32>())
                .sum()
        }
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let mut seats = seats::Seats::from_packed(input);
    let stable_occupied = seats.run_until_stable();
    println!("{}", stable_occupied);
}
