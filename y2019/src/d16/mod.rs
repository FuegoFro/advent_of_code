use itertools::Itertools;
use std::ops::Rem;
use std::{iter, mem};

const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];

fn multiplier(position: usize) -> impl Iterator<Item = i32> {
    iter::repeat(&BASE_PATTERN)
        .flat_map(move |s| {
            s.iter()
                .flat_map(move |v| iter::repeat(*v).take(position + 1))
        })
        .skip(1)
}

#[allow(dead_code)]
fn multipliers(len: usize) -> Vec<Vec<i32>> {
    (0..len)
        .map(|position| multiplier(position).take(len).collect())
        .collect()
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();
    let phases = 100;

    let mut current = iter::repeat(input.bytes().map(|b| (b - b'0') as i32))
        .take(10_000)
        .flatten()
        .collect_vec();
    let mut next = vec![0; current.len()];

    println!("Array len {}", current.len());
    // let multipliers = multipliers(current.len());

    for i in 0..phases {
        println!("Phase {}", i);
        // for (value, mult_vec) in next.iter_mut().zip_eq(multipliers.iter()) {
        for (position, value) in next.iter_mut().enumerate() {
            if position % 1000 == 0 {
                println!("Position {}", position);
            }
            *value = current
                .iter()
                .zip(multiplier(position))
                .map(|(a, b)| a * b)
                .sum::<i32>()
                .rem(10)
                .abs();
        }
        mem::swap(&mut current, &mut next);
    }

    let mut offset = 0_usize;
    for digit in current[0..7].iter() {
        offset = offset * 10 + *digit as usize;
    }

    for num in current[offset..offset + 8].iter() {
        print!("{}", num);
    }
    println!();
}
