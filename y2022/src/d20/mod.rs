use itertools::Itertools;
use skiplist::SkipList;
use std::collections::HashSet;
use util::p_i64;

fn decrypt_key(mut list: SkipList<(usize, i64)>, num_rounds: usize) -> i64 {
    let modulus = list.len() as i64;

    for _ in 0..num_rounds {
        for orig_index in 0..list.len() {
            let start_list_index = list.iter().position(|(idx, _)| *idx == orig_index).unwrap();
            let (_, item) = list.remove(start_list_index);

            let mut end_list_index = start_list_index as i64 + item;
            end_list_index = end_list_index.rem_euclid(modulus - 1);

            list.insert((orig_index, item), end_list_index as usize);
        }
    }

    let zero_index = list.iter().position(|(_, i)| *i == 0).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|offset| {
            let index = (zero_index + offset).rem_euclid(list.len());
            let (_, value) = list[index];
            value
        })
        .sum::<i64>()
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let list = input.split('\n').map(p_i64).enumerate().collect_vec();

    let pt1 = decrypt_key(list.clone().into_iter().collect(), 1);

    println!("Part 1: {}", pt1);

    let pt2 = decrypt_key(
        list.into_iter()
            .map(|(idx, item)| (idx, item * 811589153))
            .collect(),
        10,
    );
    println!("Part 2: {}", pt2);
}
