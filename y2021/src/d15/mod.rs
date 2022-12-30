use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use util::grid::{Grid, Neighbors};
use util::p_u32;
use util::point2::{Delta, PointU};

// Taken from https://doc.rust-lang.org/std/collections/binary_heap/index.html
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: PointU,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.x.cmp(&other.position.x))
            .then_with(|| self.position.y.cmp(&other.position.y))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_cheapest_path_cost(grid: &Grid<u32>) -> u32 {
    let target = PointU::new(grid.width() - 1, grid.height() - 1);
    let mut visited = HashSet::new();
    let mut frontier = BinaryHeap::from([State {
        cost: 0,
        position: PointU::ORIGIN,
    }]);
    while let Some(State { cost, position }) = frontier.pop() {
        if position == target {
            return cost;
        }
        if visited.contains(&position) {
            continue;
        }
        visited.insert(position);
        for (neighbor_position, neighbor_cost) in
            grid.neighbors_with_values(position, Neighbors::Four)
        {
            frontier.push(State {
                position: neighbor_position,
                cost: cost + *neighbor_cost,
            });
        }
    }
    panic!("Unable to find path to {:?}", target);
}

fn make_expanded_grid(orig_grid: &Grid<u32>) -> Grid<u32> {
    let mut expanded_grid = Grid::empty(orig_grid.width() * 5, orig_grid.height() * 5);
    for copy_x in 0..5 {
        for copy_y in 0..5 {
            let offset = Delta::new(
                copy_x * orig_grid.width() as i32,
                copy_y * orig_grid.height() as i32,
            );
            let value_change = (copy_x + copy_y) as u32;
            for (p, v) in orig_grid.iter_with_points() {
                let new_value = (*v + value_change - 1) % 9 + 1;
                expanded_grid[p + offset] = new_value;
            }
        }
    }
    expanded_grid
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace("\r", "");

    let grid = Grid::from_str(input, "\n", None, p_u32);

    println!("Part 1: {}", find_cheapest_path_cost(&grid));

    let expanded_grid = make_expanded_grid(&grid);
    println!("Part 2: {}", find_cheapest_path_cost(&expanded_grid));
}
