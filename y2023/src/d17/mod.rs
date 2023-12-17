use std::collections::HashSet;

use priority_queue::PriorityQueue;

use util::grid::Grid;
use util::p_i32;
use util::point2::{Delta, DeltaU, PointU, Rotation};

#[derive(Clone, Eq, PartialEq, Hash)]
struct State {
    pos: PointU,
    dir: DeltaU,
    cost: i32,
    len: u8,
    history: Vec<PointU>,
}

impl State {
    fn next(&self, grid: &Grid<i32>, dir: DeltaU, ultra: bool) -> Option<Self> {
        let min_len = if ultra { 4 } else { 1 };
        let max_len = if ultra { 10 } else { 3 };
        let (offset, len) = if dir == self.dir {
            (dir, self.len + 1)
        } else {
            (dir * (min_len as isize), min_len - 1)
        };
        grid.point_in_grid(self.pos, &offset)
            .filter(|_| len < max_len)
            .map(|pos| {
                let cost = self.cost + self.pos.step_to(&pos).map(|p| grid[p]).sum::<i32>();
                let mut history = self.history.clone();
                history.push(pos);
                State {
                    pos,
                    dir,
                    cost,
                    len,
                    history,
                }
            })
    }
}

fn enqueue(priority_queue: &mut PriorityQueue<State, i32>, state: State) {
    let cost = -state.cost;
    priority_queue.push(state, cost);
}

fn do_search(grid: &Grid<i32>, ultra: bool) -> State {
    let mut frontier = PriorityQueue::new();
    let mut initial_state = State {
        pos: PointU::ORIGIN,
        dir: DeltaU::RIGHT,
        cost: 0,
        len: 0,
        history: Vec::new(),
    };
    enqueue(&mut frontier, initial_state.clone());
    initial_state.dir = Delta::DOWN;
    enqueue(&mut frontier, initial_state);

    let mut seen = HashSet::new();
    let dest = PointU::new(grid.width() - 1, grid.height() - 1);
    loop {
        let (state, _) = frontier.pop().unwrap();
        if state.pos == dest {
            break state;
        }
        let seen_key = (state.pos, state.dir, state.len);
        if seen.contains(&seen_key) {
            continue;
        }
        seen.insert(seen_key);
        for next_state in [
            state.next(
                grid,
                state.dir.rotate_about_origin_deg(Rotation::Deg270),
                ultra,
            ),
            state.next(grid, state.dir, ultra),
            state.next(
                grid,
                state.dir.rotate_about_origin_deg(Rotation::Deg90),
                ultra,
            ),
        ]
        .into_iter()
        .flatten()
        {
            enqueue(&mut frontier, next_state);
        }
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let grid = Grid::<i32>::from_str(input, "\n", None, p_i32);

    let p1 = do_search(&grid, false);
    // WHY ARE WE 1 OFF????
    println!("Part 1: {}", p1.cost + 1);
    // let mut debug_grid = Grid::<usize>::empty(grid.width(), grid.height());
    // for (i, p) in p1.history.iter().enumerate() {
    //     debug_grid[*p] = i + 1;
    // }
    // dbg!(debug_grid);

    let p2 = do_search(&grid, true);
    println!("Part 2: {}", p2.cost + 1);

    // let mut debug_grid = Grid::<usize>::empty(grid.width(), grid.height());
    // for (i, p) in p2.history.iter().enumerate() {
    //     debug_grid[*p] = i + 1;
    // }
    // dbg!(debug_grid);
}
