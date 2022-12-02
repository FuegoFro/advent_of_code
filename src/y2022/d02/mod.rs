use crate::util::p_u32;
use itertools::Itertools;

#[derive(Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn from_str(s: &str) -> Self {
        match s {
            "A" | "X" => Choice::Rock,
            "B" | "Y" => Choice::Paper,
            "C" | "Z" => Choice::Scissors,
            _ => panic!("Unknown str {}", s),
        }
    }

    fn from_outcome(other: &Self, outcome: &Outcome) -> Self {
        let delta = match outcome {
            Outcome::Win => 1,
            Outcome::Draw => 0,
            Outcome::Lose => -1,
        };
        let new_order = other.order() + delta;
        let rtn = match new_order.rem_euclid(3) {
            0 => Choice::Rock,
            1 => Choice::Paper,
            2 => Choice::Scissors,
            _ => panic!("Unknown order {}", new_order),
        };
        assert_eq!(outcome, &Outcome::from_choices(&rtn, other));
        rtn
    }

    fn order(&self) -> i32 {
        match self {
            Choice::Rock => 0,
            Choice::Paper => 1,
            Choice::Scissors => 2,
        }
    }

    fn score(&self) -> u32 {
        self.order() as u32 + 1
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn from_str(s: &str) -> Self {
        match s {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Unknown str {}", s),
        }
    }
    fn from_choices(a: &Choice, b: &Choice) -> Self {
        let delta = (a.order() - b.order()).rem_euclid(3);
        match delta {
            0 => Outcome::Draw,
            1 => Outcome::Win,
            2 => Outcome::Lose,
            _ => panic!("Shouldn't be possible"),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let rounds1 = input
        .split('\n')
        .map(|l| {
            let (a, b) = l.split_once(" ").unwrap();
            (Choice::from_str(a), Choice::from_str(b))
        })
        .collect_vec();

    let total_score: u32 = rounds1
        .iter()
        .map(|(a, b)| b.score() + Outcome::from_choices(b, a).score())
        .sum();

    println!("Part 1: {}", total_score);

    let rounds2 = input
        .split('\n')
        .map(|l| {
            let (a, b) = l.split_once(" ").unwrap();
            (Choice::from_str(a), Outcome::from_str(b))
        })
        .collect_vec();

    let total_score2: u32 = rounds2
        .iter()
        .map(|(a, outcome)| Choice::from_outcome(a, outcome).score() + outcome.score())
        .sum();

    println!("Part 2: {}", total_score2);
}
