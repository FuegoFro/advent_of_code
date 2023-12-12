use itertools::Itertools;
use std::fmt::{Debug, Formatter, Write};
use util::grid::Grid;
use util::point2::PointU;

#[derive(Default, Copy, Clone, Eq, PartialEq)]
enum Cell {
    Galaxy,
    #[default]
    Blank,
}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Cell::Galaxy => '#',
            Cell::Blank => '.',
        })
    }
}

impl Cell {
    fn from_str(s: &str) -> Self {
        match s {
            "." => Self::Blank,
            "#" => Self::Galaxy,
            _ => panic!("Unknown str {}", s),
        }
    }
}

fn insertion_idx(haystack: &[usize], needle: usize) -> usize {
    haystack
        .iter()
        .position(|e| *e > needle)
        .unwrap_or(haystack.len())
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let compact_grid: Grid<Cell> = Grid::from_str(input, "\n", None, Cell::from_str);
    let empty_rows = (0..compact_grid.height())
        .filter(|y| {
            (0..compact_grid.width()).all(|x| compact_grid[PointU::new(x, *y)] == Cell::Blank)
        })
        .collect_vec();
    let empty_cols = (0..compact_grid.width())
        .filter(|x| {
            (0..compact_grid.height()).all(|y| compact_grid[PointU::new(*x, y)] == Cell::Blank)
        })
        .collect_vec();

    let galaxies = compact_grid
        .iter_with_points()
        .filter(|(_, c)| **c == Cell::Galaxy)
        .map(|(p, _)| p)
        .collect_vec();

    let p1 = calc_distances(&empty_rows, &empty_cols, &galaxies, 2);
    println!("Part 1: {}", p1);
    // let p2 = calc_distances(&empty_rows, &empty_cols, &galaxies, 10);
    // println!("Part 2: {}", p2);
    // let p2 = calc_distances(&empty_rows, &empty_cols, &galaxies, 100);
    // println!("Part 2: {}", p2);
    let p2 = calc_distances(&empty_rows, &empty_cols, &galaxies, 1000000);
    println!("Part 2: {}", p2);
}

fn calc_distances(
    empty_rows: &Vec<usize>,
    empty_cols: &Vec<usize>,
    galaxies: &[PointU],
    expansion: usize,
) -> usize {
    galaxies
        .iter()
        .enumerate()
        .flat_map(|(a_idx, a)| {
            let empty_rows = &empty_rows;
            let empty_cols = &empty_cols;
            galaxies
                .iter()
                .enumerate()
                .skip(a_idx)
                // .filter(move |(_, b)| a != *b)
                // .map(move |b| ((a - PointU::ORIGIN) - (b - PointU::ORIGIN)).l1_dist())
                .map(move |(_b_idx, b)| {
                    let dist_raw = a.x.abs_diff(b.x) + a.y.abs_diff(b.y);
                    let relevant_empty_rows =
                        insertion_idx(empty_rows, a.y).abs_diff(insertion_idx(empty_rows, b.y));
                    let relevant_empty_cols =
                        insertion_idx(empty_cols, a.x).abs_diff(insertion_idx(empty_cols, b.x));
                    let extra_dist = (expansion - 1) * (relevant_empty_rows + relevant_empty_cols);
                    // eprintln!(
                    //     "({} -> {}) {} w/ {} row {} cols",
                    //     a_idx, b_idx, dist_raw, relevant_empty_rows, relevant_empty_cols
                    // );
                    dist_raw + extra_dist
                })
        })
        .sum::<usize>()
}
