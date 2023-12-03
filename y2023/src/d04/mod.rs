use itertools::Itertools;
use recap::Recap;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Deserialize, Recap)]
#[recap(regex = r"Card +(?P<id>\d+): (?P<winning_numbers>.+) \| (?P<own_numbers>.+)")]
struct CardRaw {
    id: String,
    winning_numbers: String,
    own_numbers: String,
}

impl CardRaw {
    fn str_to_num_set(s: String) -> HashSet<u32> {
        s.split_whitespace().map(|n| n.parse().unwrap()).collect()
    }
    fn into_card(self) -> Card {
        let winning_numbers = CardRaw::str_to_num_set(self.winning_numbers);
        let own_numbers = CardRaw::str_to_num_set(self.own_numbers);
        Card {
            idx: self.id.parse::<usize>().unwrap() - 1,
            num_matches: winning_numbers.intersection(&own_numbers).count(),
        }
    }
}
struct Card {
    idx: usize,
    num_matches: usize,
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let cards = input
        .lines()
        .map(|l| l.parse::<CardRaw>().unwrap().into_card())
        .collect_vec();

    let pt1 = cards
        .iter()
        .map(|c| c.num_matches)
        .filter(|n| n > &0)
        .map(|n| 1 << (n - 1))
        .sum::<u32>();

    println!("Part 1: {}", pt1);

    let mut total = 0;
    let mut frontier = cards.iter().collect_vec();
    while let Some(next) = frontier.pop() {
        total += 1;
        for i in 1..=next.num_matches {
            frontier.push(&cards[next.idx + i]);
        }
    }

    println!("Part 2: {}", total);
}
