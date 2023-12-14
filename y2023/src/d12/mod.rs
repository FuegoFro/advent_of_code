use itertools::Itertools;
use std::cmp::min;
use std::collections::HashSet;

use util::p_usize;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Cell {
    Good,
    Bad,
    Unknown,
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Good,
            '#' => Self::Bad,
            '?' => Self::Unknown,
            _ => panic!("Unknown char {}", c),
        }
    }
}

#[derive(Clone)]
struct SpringRow {
    cells: Vec<Cell>,
    counts: Vec<usize>,
    counts_set: HashSet<usize>,
    min_count: usize,
    max_count: usize,
}

impl SpringRow {
    fn count_valid(&mut self, mut idx: usize) -> u32 {
        if !self.is_potentially_valid() {
            return 0;
        }

        while let Some(cell) = self.cells.get(idx) {
            if *cell == Cell::Unknown {
                break;
            }
            idx += 1;
        }

        if idx == self.cells.len() {
            self.is_valid() as u32
        } else {
            self.cells[idx] = Cell::Good;
            let good_count = self.count_valid(idx + 1);
            self.cells[idx] = Cell::Bad;
            let bad_count = self.count_valid(idx + 1);
            self.cells[idx] = Cell::Unknown;

            good_count + bad_count
        }
    }

    fn is_valid(&self) -> bool {
        self.cells
            .iter()
            .group_by(|c| **c)
            .into_iter()
            .filter(|(c, _)| *c == Cell::Bad)
            .map(|(_, g)| g.count())
            .collect_vec()
            == self.counts
    }

    fn is_potentially_valid(&self) -> bool {
        let run_sizes = self.run_sizes();
        let any_mismatch = !(&run_sizes.complete - &self.counts_set).is_empty();
        let too_big = run_sizes.incomplete_max > self.max_count;
        let too_small = run_sizes.shortest < self.min_count;
        if any_mismatch {
            let cells_str = self
                .cells
                .iter()
                .map(|c| match c {
                    Cell::Good => '.',
                    Cell::Bad => '#',
                    Cell::Unknown => '?',
                })
                .join("");
            eprintln!("{} {:?} {:?}", cells_str, run_sizes.complete, self.counts);
        }
        !any_mismatch && !too_big && !too_small
    }

    fn run_sizes(&self) -> RunSizes {
        let mut shortest = self.cells.len();
        let complete = self
            .cells
            .iter()
            .group_by(|c| **c != Cell::Good)
            .into_iter()
            .filter(|(relevant, _)| *relevant)
            .filter_map(|(_, group)| {
                let mut count = 0;
                let mut mixed = false;
                for c in group {
                    count += 1;
                    if *c == Cell::Unknown {
                        mixed = true;
                    }
                }
                shortest = min(shortest, count);
                (!mixed).then_some(count)
            })
            .collect();

        let incomplete_max = self
            .cells
            .iter()
            .group_by(|c| **c == Cell::Bad)
            .into_iter()
            .filter(|(relevant, _)| *relevant)
            .map(|(_, group)| group.count())
            .max()
            .unwrap_or(0);
        RunSizes {
            complete,
            incomplete_max,
            shortest,
        }
    }

    // fn run_sizes(&self, predicate: impl Fn(&Cell) -> bool) -> impl Iterator<Item = usize> + '_ {
    //     struct SizesIterator<'a, Inner: Iterator<Item = (usize, &'a Cell)>, P: Fn(&Cell) -> bool> {
    //         predicate: P,
    //         max_idx: usize,
    //         inner: Inner,
    //         included: bool,
    //         section_start_idx: usize,
    //         done: bool,
    //     }
    //     impl<'a, Inner: Iterator<Item = (usize, &'a Cell)>, P: Fn(&Cell) -> bool> Iterator for SizesIterator<'a, Inner, P> {
    //         type Item = usize;
    //
    //         fn next(&mut self) -> Option<Self::Item> {
    //             if self.done {
    //                 return None;
    //             }
    //             for (idx, c) in self.inner {
    //                 let new_included = (self.predicate)(c);
    //                 if new_included != self.included {
    //                     let diff = idx - self.section_start_idx;
    //                     self.section_start_idx = idx;
    //                     self.included = new_included;
    //                     if !new_included {
    //                         return Some(diff);
    //                     }
    //                 }
    //             }
    //             self.done = true;
    //             if self.included {
    //                 Some(self.max_idx - self.section_start_idx)
    //             } else {
    //                 None
    //             }
    //         }
    //     }
    //     // let mut excluded = self.cells[0] == not;
    //     // let mut section_start_idx = 0;
    //     // for (idx, c) in self.cells.iter().enumerate() {
    //     //     let new_excluded = *c == not;
    //     //     if new_excluded != excluded {
    //     //         if !excluded {
    //     //             // YIELD idx - section_start_idx
    //     //         }
    //     //         section_start_idx = idx;
    //     //         excluded = new_excluded;
    //     //     }
    //     // }
    //     SizesIterator {
    //         not,
    //         max_idx: self.cells.len(),
    //         inner: self.cells.iter().enumerate(),
    //         included: self.cells[0] == not,
    //         section_start_idx: 0,
    //         done: false,
    //     }
    // }
}

struct RunSizes {
    // complete bads - any not in list
    complete: HashSet<usize>,
    // incomplete bads - any too long
    incomplete_max: usize,
    // bads with unknowns - any too short
    shortest: usize,
}

pub fn main() {
    let input = include_str!("example_input.txt").trim().replace('\r', "");
    // let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let spring_rows = input
        .lines()
        .map(|l| {
            let (cells_raw, counts_raw) = l.split_once(' ').unwrap();
            let cells = cells_raw.chars().map(Cell::from_char).collect_vec();
            let counts = counts_raw.split(',').map(p_usize).collect_vec();
            SpringRow {
                cells,
                counts_set: counts.iter().cloned().collect(),
                min_count: *counts.iter().min().unwrap(),
                max_count: *counts.iter().max().unwrap(),
                counts,
            }
        })
        .collect_vec();

    let p1 = spring_rows
        .iter()
        .map(|r| r.clone().count_valid(0))
        .sum::<u32>();

    println!("Part 1: {}", p1);
    // println!("Part 2: {}", 2);
}
