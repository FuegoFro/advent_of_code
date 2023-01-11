use crate::d24::Cell::Blizzard;
use priority_queue::PriorityQueue;
use std::collections::HashSet;
use util::grid::Grid;
use util::point2::{Delta, PointU};

#[derive(Clone, Hash, Eq, PartialEq)]
struct Directions {
    bitmap: u8,
}

impl Directions {
    fn from_str(s: &str) -> Self {
        let mut directions = Self { bitmap: 0 };
        let direction = match s {
            "." => return directions,
            "^" => Delta::UP,
            ">" => Delta::RIGHT,
            "v" => Delta::DOWN,
            "<" => Delta::LEFT,
            _ => panic!("Unknown direction {}", s),
        };
        directions.insert(&direction);
        directions
    }

    fn mask_for_direction(direction: &Delta) -> u8 {
        let offset = match direction {
            &Delta::UP => 0,
            &Delta::RIGHT => 1,
            &Delta::DOWN => 2,
            &Delta::LEFT => 3,
            _ => panic!("Unknown direction {:?}", direction),
        };
        1 << offset
    }

    fn insert(&mut self, direction: &Delta) {
        self.bitmap |= Directions::mask_for_direction(direction);
    }

    fn contains(&self, direction: &Delta) -> bool {
        let masked = self.bitmap & Directions::mask_for_direction(direction);
        masked != 0
    }

    fn is_empty(&self) -> bool {
        self.bitmap == 0
    }

    fn clear(&mut self) {
        self.bitmap = 0;
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
enum Cell {
    Wall,
    Blizzard(Directions),
}

impl Cell {
    fn from_str(s: &str) -> Self {
        match s {
            "#" => Self::Wall,
            _ => Blizzard(Directions::from_str(s)),
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct State {
    minutes: i32,
    position: PointU,
    grid: Grid<Cell>,
}

fn advance(grid: &Grid<Cell>) -> Grid<Cell> {
    let mut next_grid = grid.clone();
    for c in next_grid.iter_mut() {
        if let Blizzard(directions) = c {
            directions.clear();
        }
    }
    for (p, c) in grid.iter_with_points() {
        if let Blizzard(directions) = c {
            for d in Delta::NEIGHBORS4 {
                if directions.contains(&d) {
                    let mut next_point = p + d;
                    if grid[next_point] == Cell::Wall {
                        next_point = match d {
                            Delta::UP => PointU::new(p.x, grid.height() - 2),
                            Delta::DOWN => PointU::new(p.x, 1),
                            Delta::LEFT => PointU::new(grid.width() - 2, p.y),
                            Delta::RIGHT => PointU::new(1, p.y),
                            _ => panic!("Unknown delta {:?}", d),
                        }
                    }
                    if let Blizzard(directions) = &mut next_grid[next_point] {
                        directions.insert(&d)
                    } else {
                        panic!(
                            "Expected to not have a wall here! {:?} -> {:?}",
                            p, next_point
                        );
                    }
                }
            }
        }
    }
    next_grid
}

fn enqueue(frontier: &mut PriorityQueue<State, i32>, state: State, target: &PointU) {
    let min_remaining = (target - state.position).l1_dist();
    let priority = state.minutes + min_remaining;
    frontier.push(state, -priority);
}

const NEXT_DELTAS: [Delta; 5] = [
    Delta::UP,
    Delta::LEFT,
    Delta::RIGHT,
    Delta::DOWN,
    Delta::NONE,
];

fn do_search(initial_state: State, end: &PointU) -> State {
    let mut frontier: PriorityQueue<State, i32> = PriorityQueue::new();
    enqueue(&mut frontier, initial_state, end);
    let mut visited = HashSet::new();
    while let Some((state, _)) = frontier.pop() {
        if visited.contains(&state) {
            continue;
        }
        if &state.position == end {
            return state;
        }
        let next_grid = advance(&state.grid);
        for next_delta in NEXT_DELTAS {
            let next_position = if let Some(next_position) = state.position.checked_add(&next_delta)
            {
                next_position
            } else {
                continue;
            };
            if next_position.x >= next_grid.width() || next_position.y >= next_grid.height() {
                continue;
            }
            if let Blizzard(directions) = &next_grid[next_position] {
                if directions.is_empty() {
                    enqueue(
                        &mut frontier,
                        State {
                            position: next_position,
                            minutes: state.minutes + 1,
                            grid: next_grid.clone(),
                        },
                        &end,
                    );
                }
            }
        }

        visited.insert(state);
    }
    panic!("Unable to find path to end!");
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let initial_grid = Grid::from_str(input, "\n", None, Cell::from_str);
    let start = PointU::new(1, 0);
    let end = PointU::new(initial_grid.width() - 2, initial_grid.height() - 1);
    let search1 = do_search(
        State {
            minutes: 0,
            position: start.clone(),
            grid: initial_grid,
        },
        &end,
    );

    println!("Part 1: {}", search1.minutes);

    let search2 = do_search(search1, &start);
    let search3 = do_search(search2, &end);
    println!("Part 2: {}", search3.minutes);
}
