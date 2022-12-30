use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use util::p_u32;

struct Tile {
    id: u32,
    possible_edges: HashSet<u16>,
}

fn bools_to_u16<'a>(i: impl Iterator<Item = &'a bool>) -> u16 {
    let mut result = 0;
    for bit in i {
        result <<= 2;
        if *bit {
            result += 1;
        }
    }
    result
}

impl Tile {
    fn from_packed(packed: &str) -> Self {
        let mut split = packed.split('\n');
        lazy_static! {
            static ref RE_TILE_ID: Regex = Regex::new(r"^Tile (?P<id>\d+):$").unwrap();
        }
        // IntelliJ doesn't understand this without this alias.
        let re_tile_id: &Regex = &RE_TILE_ID;
        let id = re_tile_id
            .captures(split.next().unwrap())
            .unwrap()
            .name("id")
            .unwrap()
            .as_str();
        let cells = split
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => panic!("Unknown char: {}", c),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut possible_edges = HashSet::new();
        for x_first in [true, false].iter() {
            for slow in [0, cells.len() - 1].iter() {
                let possible_edge_vec = (0..cells.len())
                    .map(|fast| {
                        if *x_first {
                            cells[fast][*slow]
                        } else {
                            cells[*slow][fast]
                        }
                    })
                    .collect::<Vec<_>>();
                possible_edges.insert(bools_to_u16(possible_edge_vec.iter()));
                possible_edges.insert(bools_to_u16(possible_edge_vec.iter().rev()));
            }
        }
        Tile {
            id: p_u32(id),
            possible_edges,
        }
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let mut possible_edge_to_ids: HashMap<u16, Vec<u32>> = HashMap::new();
    let tiles = input
        .split("\n\n")
        .map(Tile::from_packed)
        .collect::<Vec<_>>();
    for t in tiles.iter() {
        for edge in t.possible_edges.iter() {
            possible_edge_to_ids.entry(*edge).or_default().push(t.id);
        }
    }
    let shared_edges_by_id = tiles
        .iter()
        .map(|t| {
            let num_shared_edges = t
                .possible_edges
                .iter()
                .filter(|e| possible_edge_to_ids[e].len() > 1)
                .count();
            (num_shared_edges, t.id)
        })
        .sorted()
        .collect::<Vec<_>>();
    println!("{:?}", shared_edges_by_id);
}
