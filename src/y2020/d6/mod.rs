use std::collections::HashSet;

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let any_yeses_sum: usize = input
        .split("\n\n")
        .map(|f| f.replace("\n", "").chars().collect::<HashSet<_>>().len())
        .sum();
    println!("{}", any_yeses_sum);

    let all_yeses_sum: usize = input
        .split("\n\n")
        .map(|f| {
            let mut yeses = f.split("\n").map(|l| l.chars().collect::<HashSet<_>>());
            let first = yeses
                .next()
                .expect(&format!("'Form' has no entries: {}", f));
            yeses.fold(first, |so_far, next| &so_far & &next).len()
        })
        .sum();
    println!("{}", all_yeses_sum);
}
