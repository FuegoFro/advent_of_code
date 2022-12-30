use itertools::Itertools;
use std::fmt::{Debug, Formatter, Write};
use std::io::BufRead;
use tuple::Map;
use util::grid::Grid;
use util::point2::{Delta, PointU};
use util::{p_u32, p_usize};

#[derive(Eq, PartialEq, Clone)]
enum State {
    Air,
    Rock,
    Sand,
}

impl Default for State {
    fn default() -> Self {
        Self::Air
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            State::Air => '.',
            State::Rock => '#',
            State::Sand => 'O',
        };
        f.write_char(c)
    }
}

const SPAWN: PointU = PointU::new(500, 0);
const NEXT_POSITION_DELTAS: [Delta; 3] = [Delta::DOWN, Delta::DOWN_LEFT, Delta::DOWN_RIGHT];

fn do_single_grain(grid: &mut Grid<State>) -> bool {
    let mut current = SPAWN;
    if grid[current] == State::Sand {
        return false;
    }
    loop {
        if let Some(next) = NEXT_POSITION_DELTAS
            .iter()
            .filter_map(|d| current.checked_add(d))
            .find(|p| *grid.get(*p).unwrap_or(&State::Rock) == State::Air)
        {
            current = next;
        } else if grid.get(current + Delta::DOWN).is_some() {
            grid[current] = State::Sand;
            return true;
        } else {
            return false;
        }
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let paths = input
        .split('\n')
        .map(|l| {
            l.split(" -> ")
                .map(|p| {
                    let (x, y) = p.split_once(',').unwrap().map(p_usize);
                    PointU::new(x, y)
                })
                .collect_vec()
        })
        .collect_vec();

    let (_, max) = PointU::get_bounding_box(paths.iter().flatten().chain(std::iter::once(&SPAWN)));

    let mut grid = Grid::empty(1_000, max.y + 3);
    for path in paths.iter() {
        for (a, b) in path.iter().tuple_windows() {
            let direction = b - a;
            let count = direction.l1_dist();
            let unit_direction = direction / count;
            for i in 0..=count {
                grid[a + i * unit_direction] = State::Rock;
            }
        }
    }

    let mut grid_p1 = grid.clone();

    let mut num_grains = 0;
    // dbg!(&grid_p1);
    while do_single_grain(&mut grid_p1) {
        // dbg!(&grid_p1);
        num_grains += 1;
    }
    // dbg!(&grid_p1);

    println!("Part 1: {}", num_grains);

    let mut grid_p2 = grid.clone();
    for x in 0..grid_p2.width() {
        grid_p2[PointU::new(x, max.y + 2)] = State::Rock;
    }

    let mut num_grains = 0;
    // dbg!(&grid_p2);
    while do_single_grain(&mut grid_p2) {
        // dbg!(&grid_p2);
        num_grains += 1;
    }
    // dbg!(&grid_p2);

    println!("Part 2: {}", num_grains);
}
