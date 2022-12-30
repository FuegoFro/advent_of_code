#![allow(clippy::needless_question_mark)]
use itertools::Itertools;
use recap::Recap;
use serde::Deserialize;

#[derive(Deserialize, Recap, Debug)]
#[recap(regex = r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)")]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn from_str(s: &str) -> Self {
        s.parse().unwrap()
    }
}

#[derive(Debug, Clone)]
struct Crates {
    crates: Vec<Vec<char>>,
}

impl Crates {
    fn from_str(s: &str) -> Self {
        let lines = s.split('\n').collect_vec();
        let num_cols = (lines.last().unwrap().len() + 2) / 4;
        let mut crates = vec![Vec::new(); num_cols];

        for row in lines.iter().rev().skip(1) {
            for (col, crate_col) in crates.iter_mut().enumerate() {
                let col_idx = (col * 4) + 1;
                if col_idx >= row.len() {
                    continue;
                }
                let val = row.chars().nth(col_idx).unwrap();
                if val == ' ' {
                    continue;
                }
                crate_col.push(val);
            }
        }

        Self { crates }
    }

    fn do_move(&mut self, m: &Move, preserve_order: bool) {
        let from_len = self.crates[m.from - 1].len();
        let mut to_move = self.crates[m.from - 1].split_off(from_len - m.count);
        if !preserve_order {
            to_move.reverse();
        }
        self.crates[m.to - 1].append(&mut to_move);
    }

    fn top_letters(&self) -> String {
        self.crates.iter().map(|c| c.last().unwrap()).join("")
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt")
    //     .trim_end()
    //     .replace("\r", "");
    let input = include_str!("actual_input.txt")
        .trim_end()
        .replace('\r', "");

    let (start_str, moves_str) = input.split_once("\n\n").unwrap();
    let mut crates1 = Crates::from_str(start_str);
    let mut crates2 = crates1.clone();
    let moves = moves_str.split('\n').map(Move::from_str).collect_vec();

    for m in moves.iter() {
        crates1.do_move(m, false);
    }

    println!("Part 1: {}", crates1.top_letters());

    for m in moves.iter() {
        crates2.do_move(m, true);
    }

    println!("Part 2: {}", crates2.top_letters());
}
