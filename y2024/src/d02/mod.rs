use itertools::Itertools;

use util::p_i32;

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let reports = input
        .lines()
        .map(|l| l.split_whitespace().map(p_i32).collect_vec())
        .collect_vec();

    let p1 = reports
        .iter()
        .filter(|r| {
            let diffs = r.iter().tuple_windows().map(|(a, b)| b - a).collect_vec();
            diffs
                .iter()
                .all(|&d| (-3..=-1).contains(&d) || (1..=3).contains(&d))
                && (diffs.iter().all(|&d| d > 0) || diffs.iter().all(|&d| d < 0))
        })
        .count();

    println!("Part 1: {}", p1);

    let p2 = p2(&reports);
    println!("Part 2: {}", p2);
}

fn p2(reports: &[Vec<i32>]) -> usize {
    let mut num_valid = 0;
    for r in reports.iter() {
        for to_skip in 0..r.len() {
            let mut prev_element: Option<i32> = None;
            let mut expect_positive: Option<bool> = None;
            let mut is_valid = true;
            for (idx, element) in r.iter().enumerate() {
                if idx == to_skip {
                    continue;
                }
                if let Some(prev_element) = prev_element {
                    let diff = element - prev_element;
                    if !(1..=3).contains(&diff.abs()) {
                        is_valid = false;
                        break;
                    }
                    let current_positive = diff > 0;
                    if let Some(expect_positive) = expect_positive {
                        if current_positive != expect_positive {
                            is_valid = false;
                            break;
                        }
                    }
                    expect_positive = Some(current_positive);
                }
                prev_element = Some(*element);
            }
            if is_valid {
                num_valid += 1;
                break;
            }
        }
    }

    num_valid
}
