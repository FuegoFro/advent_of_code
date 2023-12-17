use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use itertools::Itertools;
use serde::Serialize;

use util::p_usize;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Serialize)]
enum Cell {
    #[serde(rename = ".")]
    Good,
    #[serde(rename = "#")]
    Bad,
    #[serde(rename = "?")]
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

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = serde_json::to_string(self).unwrap();
        f.write_str(&string[1..(string.len() - 1)])
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct MemoKey {
    cell_idx: usize,
    count_idx: usize,
    current_bad_count: Option<usize>,
    current_cell: Cell,
}

#[derive(Clone)]
struct SpringRow {
    cells: Vec<Cell>,
    counts: Vec<usize>,
    memo: HashMap<MemoKey, u64>,
}

impl SpringRow {
    fn expanded(&self) -> Self {
        Self {
            cells: self
                .cells
                .iter()
                .cloned()
                .chain([Cell::Unknown])
                .cycle()
                .take(self.cells.len() * 5 + 4)
                .collect_vec(),
            counts: self
                .counts
                .iter()
                .cloned()
                .cycle()
                .take(self.counts.len() * 5)
                .collect_vec(),
            memo: HashMap::new(),
        }
    }

    fn count_valid(
        &mut self,
        mut cell_idx: usize,
        mut count_idx: usize,
        mut current_bad_count: Option<usize>,
    ) -> u64 {
        // everything before cell_idx is known
        // previous is either good or bad
        // current_bad_len = None iff prev is good

        while let Some(cell) = self.cells.get(cell_idx) {
            match cell {
                Cell::Good => {
                    if current_bad_count.is_some() {
                        if current_bad_count.as_ref() != self.counts.get(count_idx) {
                            // We finished a bad run with the wrong count
                            return 0;
                        }
                        // We finished a bad run with the right len, advance to next count and reset
                        count_idx += 1;
                        current_bad_count = None;
                    }
                }
                Cell::Bad => {
                    // Increment or initialize our bad run count
                    current_bad_count = Some(current_bad_count.unwrap_or(0) + 1);
                    let invalid = self
                        .counts
                        .get(count_idx)
                        // True if our current run is too long
                        .map(|next_count| current_bad_count.unwrap() > *next_count)
                        // True if we're done with counts but ran into a bad
                        .unwrap_or(true);
                    if invalid {
                        return 0;
                    }
                }
                Cell::Unknown => {
                    // Recurse for each possibility, without advancing counts
                    let mut memo_key = MemoKey {
                        cell_idx,
                        count_idx,
                        current_bad_count,
                        current_cell: Cell::Good,
                    };
                    self.cells[cell_idx] = Cell::Good;
                    let good_count = if let Some(existing) = self.memo.get(&memo_key) {
                        *existing
                    } else {
                        let new = self.count_valid(cell_idx, count_idx, current_bad_count);
                        self.memo.insert(memo_key.clone(), new);
                        new
                    };
                    memo_key.current_cell = Cell::Bad;
                    self.cells[cell_idx] = Cell::Bad;
                    let bad_count = if let Some(existing) = self.memo.get(&memo_key) {
                        *existing
                    } else {
                        let new = self.count_valid(cell_idx, count_idx, current_bad_count);
                        self.memo.insert(memo_key.clone(), new);
                        new
                    };
                    self.cells[cell_idx] = Cell::Unknown;

                    return good_count + bad_count;
                }
            }
            cell_idx += 1;
        }
        let is_valid = (count_idx == self.counts.len() - 1 || count_idx == self.counts.len())
            && current_bad_count.as_ref() == self.counts.get(count_idx);
        // We reached the end, return a `1` iff our final state is valid
        is_valid as u64
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let spring_rows = input
        .lines()
        .map(|l| {
            let (cells_raw, counts_raw) = l.split_once(' ').unwrap();
            let cells = cells_raw.chars().map(Cell::from_char).collect_vec();
            let counts = counts_raw.split(',').map(p_usize).collect_vec();
            SpringRow {
                cells,
                counts,
                memo: HashMap::new(),
            }
        })
        .collect_vec();

    let p1 = spring_rows
        .iter()
        .map(|r| r.clone().count_valid(0, 0, None))
        .sum::<u64>();

    println!("Part 1: {}", p1);

    let p2 = spring_rows
        .iter()
        .map(|r| r.expanded().count_valid(0, 0, None))
        .sum::<u64>();
    println!("Part 2: {}", p2);
}
