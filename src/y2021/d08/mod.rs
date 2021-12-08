use itertools::Itertools;
use std::collections::{HashMap, HashSet};

struct Wire(char);

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Segment {
    TopCenter,
    TopLeft,
    TopRight,
    CenterCenter,
    BottomLeft,
    BottomRight,
    BottomCenter,
}

impl Segment {
    const ALL: [Segment; 7] = [
        Segment::TopCenter,
        Segment::TopLeft,
        Segment::TopRight,
        Segment::CenterCenter,
        Segment::BottomLeft,
        Segment::BottomRight,
        Segment::BottomCenter,
    ];
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digit {
    const ALL: [Digit; 10] = [
        Digit::Zero,
        Digit::One,
        Digit::Two,
        Digit::Three,
        Digit::Four,
        Digit::Five,
        Digit::Six,
        Digit::Seven,
        Digit::Eight,
        Digit::Nine,
    ];

    fn possible_digits(digit_count: usize) -> Vec<Digit> {
        Digit::ALL
            .iter()
            .filter(|d| d.segments().len() == digit_count)
            .cloned()
            .collect_vec()
    }

    fn get_value(&self) -> u32 {
        match self {
            Digit::Zero => 0,
            Digit::One => 1,
            Digit::Two => 2,
            Digit::Three => 3,
            Digit::Four => 4,
            Digit::Five => 5,
            Digit::Six => 6,
            Digit::Seven => 7,
            Digit::Eight => 8,
            Digit::Nine => 9,
        }
    }

    fn segments(&self) -> Vec<Segment> {
        match self {
            Digit::Zero => vec![
                Segment::TopCenter,
                Segment::TopLeft,
                Segment::TopRight,
                Segment::BottomLeft,
                Segment::BottomRight,
                Segment::BottomCenter,
            ],
            Digit::One => vec![Segment::TopRight, Segment::BottomRight],
            Digit::Two => vec![
                Segment::TopCenter,
                Segment::TopRight,
                Segment::CenterCenter,
                Segment::BottomLeft,
                Segment::BottomCenter,
            ],
            Digit::Three => vec![
                Segment::TopCenter,
                Segment::TopRight,
                Segment::CenterCenter,
                Segment::BottomRight,
                Segment::BottomCenter,
            ],
            Digit::Four => vec![
                Segment::TopLeft,
                Segment::TopRight,
                Segment::CenterCenter,
                Segment::BottomRight,
            ],
            Digit::Five => vec![
                Segment::TopCenter,
                Segment::TopLeft,
                Segment::CenterCenter,
                Segment::BottomRight,
                Segment::BottomCenter,
            ],
            Digit::Six => vec![
                Segment::TopCenter,
                Segment::TopLeft,
                Segment::CenterCenter,
                Segment::BottomLeft,
                Segment::BottomRight,
                Segment::BottomCenter,
            ],
            Digit::Seven => vec![Segment::TopCenter, Segment::TopRight, Segment::BottomRight],
            Digit::Eight => vec![
                Segment::TopCenter,
                Segment::TopLeft,
                Segment::TopRight,
                Segment::CenterCenter,
                Segment::BottomLeft,
                Segment::BottomRight,
                Segment::BottomCenter,
            ],
            Digit::Nine => vec![
                Segment::TopCenter,
                Segment::TopLeft,
                Segment::TopRight,
                Segment::CenterCenter,
                Segment::BottomRight,
                Segment::BottomCenter,
            ],
        }
    }
}

struct WireTracker {
    wire_to_segment_possibilities: HashMap<Wire, HashSet<Segment>>,
}

fn decode_entry(signals: &Vec<&str>, digits: &Vec<&str>) -> u32 {
    let mut wire_to_segment_possibilities: HashMap<char, HashSet<Segment>> = HashMap::new();
    for wire in 'a'..='g' {
        wire_to_segment_possibilities.insert(wire, Segment::ALL.iter().cloned().collect());
    }
    for signal in signals {
        dbg!(&wire_to_segment_possibilities);
        dbg!(signal);
        let mut possible_segments = HashSet::new();
        for digit in Digit::possible_digits(signal.len()) {
            dbg!(&digit);
            possible_segments = possible_segments
                .union(&(digit.segments().into_iter().collect()))
                .cloned()
                .collect();
        }
        dbg!(&possible_segments);
        for wire in signal.chars() {
            let entry = wire_to_segment_possibilities.get_mut(&wire).unwrap();
            *entry = entry.intersection(&possible_segments).cloned().collect();
        }
    }
    dbg!(&wire_to_segment_possibilities);
    0
}

/*

For each wire, track possible positions
Start off all wires as all possible positions
On each digit, intersect the set of possible wires for the used signals with the known possible
Once each set has size 1, translate the result

NOPE

2 -> 1
4 -> 4
3 -> 7
7 -> 8


6
    Overlap w/ 4 -> 9
    Else overlap w/ 1 -> 0 (can determine center center wire)
    Else -> 6

5
    Overlap w/ 7 -> 3 (can determine top center wire)
    Overlaps 9 -> 5
    Else 2

*/

fn decode_entry2(signals: &Vec<&str>, queries: &Vec<&str>) -> u32 {
    let mut signals_by_length: HashMap<usize, Vec<HashSet<char>>> = HashMap::new();
    for signal in signals.iter() {
        signals_by_length
            .entry(signal.len())
            .or_default()
            .push(signal.chars().collect());
    }
    let mut signals_by_digit: HashMap<Digit, HashSet<char>> = HashMap::new();
    signals_by_digit.insert(
        Digit::One,
        signals_by_length.remove(&2).unwrap().pop().unwrap(),
    );
    signals_by_digit.insert(
        Digit::Four,
        signals_by_length.remove(&4).unwrap().pop().unwrap(),
    );
    signals_by_digit.insert(
        Digit::Seven,
        signals_by_length.remove(&3).unwrap().pop().unwrap(),
    );
    signals_by_digit.insert(
        Digit::Eight,
        signals_by_length.remove(&7).unwrap().pop().unwrap(),
    );
    for len_6 in signals_by_length.remove(&6).unwrap() {
        if signals_by_digit
            .get(&Digit::Four)
            .unwrap()
            .is_subset(&len_6)
        {
            signals_by_digit.insert(Digit::Nine, len_6);
        } else if signals_by_digit.get(&Digit::One).unwrap().is_subset(&len_6) {
            signals_by_digit.insert(Digit::Zero, len_6);
        } else {
            signals_by_digit.insert(Digit::Six, len_6);
        }
    }
    for len_5 in signals_by_length.remove(&5).unwrap() {
        if signals_by_digit
            .get(&Digit::Seven)
            .unwrap()
            .is_subset(&len_5)
        {
            signals_by_digit.insert(Digit::Three, len_5);
        } else if len_5.is_subset(signals_by_digit.get(&Digit::Nine).unwrap()) {
            signals_by_digit.insert(Digit::Five, len_5);
        } else {
            signals_by_digit.insert(Digit::Two, len_5);
        }
    }

    let mut total = 0;
    for raw_query in queries.iter() {
        let query: HashSet<char> = raw_query.chars().collect();
        let digit = signals_by_digit
            .iter()
            .filter_map(|(d, s)| {
                if *s == query {
                    Some(d.get_value())
                } else {
                    None
                }
            })
            .next()
            .unwrap();
        total = total * 10 + digit;
    }
    total
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace("\r", "");

    let raw_entries = input
        .split("\n")
        .map(|e| e.split_once(" | ").unwrap())
        .map(|(signals, digits)| {
            (
                signals.split(" ").collect_vec(),
                digits.split(" ").collect_vec(),
            )
        })
        .collect_vec();

    let num_unique_digits = raw_entries
        .iter()
        .flat_map(|(_, e)| {
            e.iter().filter(|d| {
                let d_len = d.len();
                d_len == 2 || d_len == 4 || d_len == 3 || d_len == 7
            })
        })
        .count();

    println!("Part 1: {}", num_unique_digits);

    let all_queries: u32 = raw_entries.iter().map(|(s, q)| decode_entry2(s, q)).sum();
    // let first_entry = decode_entry2(&raw_entries[0].0, &raw_entries[0].1);
    // dbg!(first_entry);
    println!("Part 2: {}", all_queries);
}
