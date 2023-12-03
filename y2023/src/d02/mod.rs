use itertools::Itertools;
use recap::Recap;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"Game (?P<id>\d+): (?P<observations>.+)")]
struct GameRaw {
    id: u32,
    observations: String,
}

#[derive(Debug)]
struct Game {
    id: u32,
    observations: Vec<Observation>,
}

impl Game {
    fn is_valid(&self, maximums: &HashMap<&str, u32>) -> bool {
        self.observations.iter().all(|o| {
            o.color_infos.iter().all(|ci| {
                maximums
                    .get(ci.color.as_str())
                    .map(|m| m >= &ci.count)
                    .unwrap_or(false)
            })
        })
    }
}

#[derive(Debug)]
struct Observation {
    color_infos: Vec<ColorInfo>,
}

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"(?P<count>\d+) (?P<color>\w+)")]
struct ColorInfo {
    color: String,
    count: u32,
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let games = input
        .lines()
        .map(|l| {
            let game_raw = l.parse::<GameRaw>().unwrap();
            Game {
                id: game_raw.id,
                observations: game_raw
                    .observations
                    .split("; ")
                    .map(|o| Observation {
                        color_infos: o.split(", ").map(|ci| ci.parse().unwrap()).collect(),
                    })
                    .collect(),
            }
        })
        .collect_vec();

    let maximums = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let pt1 = games
        .iter()
        .filter(|g| g.is_valid(&maximums))
        .map(|g| g.id)
        .sum::<u32>();

    println!("Part 1: {}", pt1);

    let pt2 = games
        .iter()
        .map(|g| {
            g.observations
                .iter()
                .fold(HashMap::new(), |mut hm, o| {
                    for ci in o.color_infos.iter() {
                        if hm.get(ci.color.as_str()).unwrap_or(&0) <= &ci.count {
                            hm.insert(ci.color.as_str(), ci.count);
                        }
                    }
                    hm
                })
                .into_values()
                .product::<u32>()
        })
        .sum::<u32>();

    println!("Part 2: {}", pt2);
}
