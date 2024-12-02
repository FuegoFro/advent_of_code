use itertools::Itertools;
use util::p_u32;

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let (mut a_nums, mut b_nums): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(p_u32)
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .unzip();

    a_nums.sort();
    b_nums.sort();

    let p1 = a_nums
        .iter()
        .zip(&b_nums)
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<u32>();

    println!("Part 1: {}", p1);

    let b_counts = b_nums.iter().counts();
    let p2 = a_nums
        .iter()
        .map(|a| *a as usize * b_counts.get(a).unwrap_or(&0))
        .sum::<usize>();

    println!("Part 2: {}", p2);
}
