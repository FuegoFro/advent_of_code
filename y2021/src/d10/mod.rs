use itertools::{Either, Itertools};

fn get_corresponding_close(c: char) -> Option<char> {
    Some(match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => return None,
    })
}

fn parse_first_incorrect_or_necessary_remaining(line: &str) -> Either<char, Vec<char>> {
    let mut close_stack = vec![];
    for c in line.chars() {
        if let Some(closer) = get_corresponding_close(c) {
            close_stack.push(closer)
        } else if close_stack.pop() != Some(c) {
            return Either::Left(c);
        }
    }
    Either::Right(close_stack)
}

fn find_first_incorrect_char(line: &str) -> Option<char> {
    parse_first_incorrect_or_necessary_remaining(line).left()
}

fn get_remaining_closers(line: &str) -> Option<Vec<char>> {
    parse_first_incorrect_or_necessary_remaining(line).right()
}

fn points_for_incorrect_closers(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Unknown closer {}", c),
    }
}

fn points_for_remaining_closers(closers: Vec<char>) -> u64 {
    closers.iter().rev().fold(0, |total, c| {
        total * 5
            + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!("Unknown closer {}", c),
            }
    })
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace("\r", "");

    let lines = input.split("\n");

    let total_syntax_error_score: u64 = lines
        .clone()
        .flat_map(find_first_incorrect_char)
        .map(points_for_incorrect_closers)
        .sum();
    println!("Part 1: {}", total_syntax_error_score);

    let scores = lines
        .flat_map(get_remaining_closers)
        .map(points_for_remaining_closers)
        .sorted()
        .collect_vec();
    println!("Part 2: {}", scores[scores.len() / 2]);
}
