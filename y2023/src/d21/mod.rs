use std::collections::HashSet;
use std::fmt::{Debug, Formatter};

use serde::{Deserialize, Serialize};

use util::grid::Grid;
use util::impl_debug_serde;
use util::point2::{DeltaU, Point, PointU};

impl_debug_serde!(Cell);
#[derive(PartialEq, Serialize, Deserialize)]
enum Cell {
    #[serde(rename = "#")]
    Empty,
    #[serde(rename = ".")]
    Garden,
    #[serde(rename = "S")]
    Start,
}

type PointW = Point<isize>;

pub fn main() {
    let input = include_str!("example_input.txt").trim().replace('\r', "");
    let steps = 5000;
    // let steps = 6;
    // let input = include_str!("actual_input.txt").trim().replace('\r', "");
    // let steps = 64;

    let mut grid = Grid::<Cell>::from_serde_chars(input);
    let start = grid
        .iter_mut_with_points()
        .filter_map(|(p, c)| {
            (*c == Cell::Start).then(|| {
                // Definitely a bit cheeky to modify the grid as a side effect here, but whatever.
                *c = Cell::Garden;
                p
            })
        })
        .next()
        .unwrap();
    // dbg!(&grid);
    let converter = Converter {
        width: grid.width() as _,
        height: grid.height() as _,
    };

    let mut seen_even = HashSet::<PointW>::new();
    let mut seen_odd = HashSet::new();

    let mut current = HashSet::new();
    current.insert(start.cast().unwrap());
    for i in 0..steps {
        if i % 1000 == 0 {
            println!("{} {}", i, current.len());
        }
        let seen = if i % 2 == 0 {
            &mut seen_even
        } else {
            &mut seen_odd
        };
        current = current
            .into_iter()
            .flat_map(|pos| {
                [
                    pos + DeltaU::UP,
                    pos + DeltaU::RIGHT,
                    pos + DeltaU::DOWN,
                    pos + DeltaU::LEFT,
                ]
                .into_iter()
                // grid.neighbors_with_values(pos, Neighbors::Four)
                //     .filter(|(_, c)| **c == Cell::Garden)
                //     .map(|(p, _)| p)
            })
            .filter(|p| !seen.contains(p) && grid[converter.world_to_grid(*p)] == Cell::Garden)
            .collect();
        // dbg!(Grid::from_points(current.iter(), None));
        // dbg!(&current);
        // for p in current.iter() {
        //     grid[converter.world_to_grid(*p)] = Cell::Start;
        // }
        // dbg!(&grid);
        // break;
        seen.extend(current.iter());
    }
    let p1 = (if steps % 2 == 1 {
        &seen_even
    } else {
        &seen_odd
    })
    .len();

    println!("Part 1: {}", p1);
    // println!("Part 2: {}", p2);
}

struct Converter {
    width: isize,
    height: isize,
}

impl Converter {
    // fn world_to_grid(&self, world: PointW) -> (DeltaS, PointU) {
    fn world_to_grid(&self, world: PointW) -> PointU {
        let grid_x = world.x.rem_euclid(self.width);
        let grid_y = world.y.rem_euclid(self.height);

        // (world - grid_relative, grid_relative.cast().unwrap())
        PointU::new(grid_x as _, grid_y as _)
    }

    // fn grid_to_world(&self, offset: DeltaS, grid: PointU) -> PointW {
    //     grid.cast().unwrap() + offset
    // }
}
