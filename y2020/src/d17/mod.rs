use crate::d17::BoardLayer::Dimension;
use itertools::Itertools;
use std::mem;

struct BoardPoint {
    coordinates: Vec<i32>,
}

#[derive(Clone)]
enum BoardLayer {
    Dimension(Vec<BoardLayer>),
    Value(bool),
}

impl BoardLayer {
    fn count_active(&self) -> u32 {
        match self {
            Dimension(children) => children.iter().map(|c| c.count_active()).sum(),
            BoardLayer::Value(v) => {
                if *v {
                    1
                } else {
                    0
                }
            }
        }
    }
}

struct Board {
    dimensions: usize,
    cells: BoardLayer,
    temp: Option<BoardLayer>,
}

impl Board {
    fn from_packed(packed: &str, dimensions: usize, max_cycles: usize) -> Self {
        assert!(dimensions >= 2);
        let initial_plane = packed
            .split("\n")
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => panic!("Unknown char: {}", c),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let sizes_xy = max_cycles * 2 + initial_plane.len();
        let mut sizes = vec![max_cycles * 2 + 1; dimensions];
        sizes[0] = sizes_xy;
        sizes[1] = sizes_xy;

        let mut layer = BoardLayer::Value(false);
        for size in sizes.iter() {
            layer = Dimension(vec![layer; *size]);
        }

        let mut board = Board {
            dimensions,
            cells: layer.clone(),
            temp: Some(layer),
        };

        // Copy initial plan into board
        for (y, initial_row) in initial_plane.iter().enumerate() {
            for (x, initial_val) in initial_row.iter().enumerate() {
                let mut point = vec![max_cycles as i32; dimensions];
                point[0] += x as i32;
                point[1] += y as i32;
                board.set(BoardPoint { coordinates: point }, *initial_val);
            }
        }

        board
    }

    fn get(&self, point: BoardPoint) -> Option<&bool> {
        assert_eq!(point.coordinates.len(), self.dimensions);
        let mut layer = Some(&self.cells);
        for coordinate in point.coordinates.into_iter().rev() {
            if coordinate < 0 {
                return None;
            }
            layer = match layer {
                Some(Dimension(children)) => children.get(coordinate as usize),
                Some(BoardLayer::Value(_)) => panic!("Ran out of dimensions earlier than expected"),
                None => return None,
            }
        }

        match layer {
            Some(Dimension(_)) => panic!("More dimensions than expected"),
            Some(BoardLayer::Value(v)) => Some(v),
            None => None,
        }
    }

    fn set(&mut self, point: BoardPoint, value: bool) {
        assert_eq!(point.coordinates.len(), self.dimensions);
        let mut layer = &mut self.cells;
        for coordinate in point.coordinates.into_iter().rev() {
            layer = match layer {
                Dimension(children) => &mut children[coordinate as usize],
                BoardLayer::Value(_) => panic!("Ran out of dimensions earlier than expected"),
            }
        }

        match layer {
            Dimension(_) => panic!("More dimensions than expected"),
            BoardLayer::Value(v) => *v = value,
        }
    }

    fn cycle_layer_helper(&self, next: &mut BoardLayer, reverse_coordinates_so_far: &mut Vec<i32>) {
        match next {
            Dimension(children) => children.iter_mut().enumerate().for_each(|(i, child)| {
                reverse_coordinates_so_far.push(i as i32);
                self.cycle_layer_helper(child, reverse_coordinates_so_far);
                reverse_coordinates_so_far.pop();
            }),
            BoardLayer::Value(next_val) => {
                let mut coordinates = reverse_coordinates_so_far.clone();
                coordinates.reverse();
                let occupied_neighbors = self.count_occupied_neighbors(BoardPoint {
                    coordinates: coordinates.clone(),
                });
                let current_val = self.get(BoardPoint { coordinates }).unwrap();
                *next_val = if *current_val {
                    occupied_neighbors == 2 || occupied_neighbors == 3
                } else {
                    occupied_neighbors == 3
                };
            }
        }
    }

    fn do_cycle(&mut self) {
        let mut temp = self.temp.take().unwrap();
        let mut coordinates = Vec::new();

        self.cycle_layer_helper(&mut temp, &mut coordinates);

        mem::swap(&mut temp, &mut self.cells);
        self.temp = Some(temp);
    }

    fn count_occupied_neighbors(&self, board_point: BoardPoint) -> u32 {
        (0..self.dimensions)
            .map(|_| (-1..=1))
            .multi_cartesian_product()
            .filter(|offsets| !offsets.iter().all(|offset| *offset == 0))
            .map(|offsets| {
                self.get(BoardPoint {
                    coordinates: offsets
                        .iter()
                        .zip(board_point.coordinates.iter())
                        .map(|(a, b)| a + b)
                        .collect(),
                })
            })
            .map(|v| match v {
                Some(true) => 1,
                _ => 0,
            })
            .sum()
    }

    fn total_active_cells(&self) -> u32 {
        self.cells.count_active()
    }

    #[allow(dead_code)]
    fn print_board(&self) {
        println!("current:");
        let mut v = Vec::new();
        self.print_board_helper(&self.cells, &mut v);
    }

    fn print_board_helper(&self, layer: &BoardLayer, reverse_coords_so_far: &mut Vec<usize>) {
        if self.dimensions == reverse_coords_so_far.len() + 2 {
            if layer.count_active() == 0 {
                return;
            }
            println!(
                "higher dimen coords (reversed): {:?}",
                reverse_coords_so_far
            );
            let plane = match layer {
                Dimension(children) => children,
                BoardLayer::Value(_) => panic!("Expected 2 more dimensions"),
            };
            for row in plane.iter() {
                let row = match row {
                    Dimension(children) => children,
                    BoardLayer::Value(_) => panic!("Expected 1 more dimension"),
                };
                for val in row.iter() {
                    let c = match val {
                        BoardLayer::Value(true) => '#',
                        BoardLayer::Value(false) => '.',
                        _ => panic!("Expected a value"),
                    };
                    print!("{}", c)
                }
                print!("\n")
            }
        } else {
            match layer {
                Dimension(children) => children.iter().enumerate().for_each(|(i, child)| {
                    reverse_coords_so_far.push(i);
                    self.print_board_helper(child, reverse_coords_so_far);
                    reverse_coords_so_far.pop();
                }),
                BoardLayer::Value(_) => panic!("Should not print with fewer than 2 dimensions"),
            }
        }
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let num_cycles = 6;

    let mut board3d = Board::from_packed(input, 3, num_cycles);
    for _ in 0..num_cycles {
        board3d.do_cycle();
    }
    println!("{}", board3d.total_active_cells());

    let mut board4d = Board::from_packed(input, 4, num_cycles);
    for _ in 0..num_cycles {
        board4d.do_cycle();
    }
    println!("{}", board4d.total_active_cells());
}
