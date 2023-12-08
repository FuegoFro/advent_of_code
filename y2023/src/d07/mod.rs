use itertools::Itertools;
use util::p_u32;

struct Hand {
    cards: Vec<u32>,
    bid: u32,
}

impl Hand {
    fn from_str(s: &str) -> Self {
        let (cards_raw, bid_raw) = s.split_once(' ').unwrap();
        let cards = cards_raw
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                '2'..='9' => c.to_digit(10).unwrap(),
                _ => panic!("Unknown card {}", c),
            })
            .collect_vec();
        let bid = p_u32(bid_raw);

        Hand { cards, bid }
    }

    fn sort_key(&self) -> Vec<u32> {
        let mut counts = self.cards.iter().counts().into_values().sorted().rev();
        let first_two_counts = (counts.next().unwrap(), counts.next().unwrap_or(0));
        let type_val = match first_two_counts {
            (5, 0) => 6,
            (4, 1) => 5,
            (3, 2) => 4,
            (3, 1) => 3,
            (2, 2) => 2,
            (2, 1) => 1,
            (1, 1) => 0,
            _ => unreachable!("{:?}", first_two_counts),
        };

        let mut result = vec![type_val];
        result.extend(self.cards.iter().to_owned());
        result
    }

    fn sort_key_wild(&self) -> Vec<u32> {
        let mut counts_map = self.cards.iter().counts();
        let wilds = counts_map.remove(&11).unwrap_or(0);
        let mut counts = counts_map.into_values().sorted().rev();
        let first_two_counts = (
            counts.next().unwrap_or(0) + wilds,
            counts.next().unwrap_or(0),
        );
        let type_val = match first_two_counts {
            (5, 0) => 6,
            (4, 1) => 5,
            (3, 2) => 4,
            (3, 1) => 3,
            (2, 2) => 2,
            (2, 1) => 1,
            (1, 1) => 0,
            _ => unreachable!("{:?}", first_two_counts),
        };

        let mut result = vec![type_val];
        result.extend(
            self.cards
                .iter()
                .cloned()
                .map(|c| if c == 11 { 1 } else { c }),
        );
        result
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let p1 = input
        .lines()
        .map(Hand::from_str)
        .sorted_by_key(|h| h.sort_key())
        .enumerate()
        .map(|(idx, h)| h.bid * (idx as u32 + 1))
        .sum::<u32>();

    println!("Part 1: {}", p1);

    let p2 = input
        .lines()
        .map(Hand::from_str)
        .sorted_by_key(|h| h.sort_key_wild())
        .enumerate()
        .map(|(idx, h)| h.bid * (idx as u32 + 1))
        .sum::<u32>();

    println!("Part 2: {}", p2);
}
