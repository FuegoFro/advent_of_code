use itertools::Itertools;
use util::p_i32;

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let sequences = input
        .lines()
        .map(|l| l.split_whitespace().map(p_i32).collect_vec())
        .collect_vec();

    let p1 = sequences.iter().map(|s| predict_next(s)).sum::<i32>();

    println!("Part 1: {}", p1);

    let p2 = sequences
        .into_iter()
        .map(|mut s| {
            s.reverse();
            predict_next(&s)
        })
        .sum::<i32>();
    println!("Part 2: {}", p2);
}

fn predict_next(sequence: &[i32]) -> i32 {
    if sequence.iter().all(|e| *e == 0) {
        0
    } else {
        let derivative = sequence
            .iter()
            .fold((vec![], None), |(mut deriv, prev), next| {
                if let Some(prev) = prev {
                    deriv.push(next - prev);
                }
                (deriv, Some(*next))
            })
            .0;

        let derivative_next = predict_next(&derivative);
        sequence.last().unwrap() + derivative_next
    }
}
