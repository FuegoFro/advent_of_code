use std::fmt::{Debug, Formatter, Write};
use util::grid::Grid;
use util::point2::Delta;
use util::point2::PointU;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    Right,
    Down,
}

impl Direction {
    fn delta(&self) -> Delta<isize> {
        match self {
            Direction::Right => Delta::RIGHT,
            Direction::Down => Delta::DOWN,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Spot {
    Empty,
    Dir(Direction),
}

impl Spot {
    fn from_str(s: &str) -> Self {
        match s {
            "." => Spot::Empty,
            ">" => Spot::Dir(Direction::Right),
            "v" => Spot::Dir(Direction::Down),
            _ => panic!("Unknown spot str: {}", s),
        }
    }
}

impl Debug for Spot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Spot::Empty => f.write_char('.'),
            Spot::Dir(dir) => match dir {
                Direction::Right => f.write_char('>'),
                Direction::Down => f.write_char('v'),
            },
        }
    }
}

fn move_one_type(grid: Grid<Spot>, dir: Direction) -> (Grid<Spot>, bool) {
    let mut result = grid.clone();
    let target_spot = Spot::Dir(dir);

    for p in grid.points() {
        if grid[p] != target_spot {
            continue;
        }
        let orig_target = p + dir.delta();
        let target = PointU::new(orig_target.x % grid.width(), orig_target.y % grid.height());
        if grid[target] == Spot::Empty {
            result[target] = grid[p];
            result[p] = Spot::Empty;
        }
    }
    let changed = grid != result;
    (result, changed)
}

fn do_time_step(grid: Grid<Spot>) -> (Grid<Spot>, bool) {
    // Don't short circuit
    let (grid, did_move_right) = move_one_type(grid, Direction::Right);
    let (grid, did_move_down) = move_one_type(grid, Direction::Down);
    (grid, did_move_right || did_move_down)
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let mut grid = Grid::from_str(input, "\n", None, Spot::from_str);

    let mut num_steps = 0;
    let mut did_work = true;
    while did_work {
        let (new_grid, new_did_work) = do_time_step(grid);
        grid = new_grid;
        did_work = new_did_work;
        num_steps += 1;
    }
    println!("Part 1: {}", num_steps);
}
