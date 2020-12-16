use crate::util::p_u64;

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let mut parts = input.split("\n");
    let current_time = parts.next().map(p_u64).unwrap();
    let buses_raw = parts.next().unwrap();

    let (wait_time, bus_id) = buses_raw
        .split(",")
        .filter(|id| *id != "x")
        .map(p_u64)
        .map(|id| (id - (current_time % id), id))
        .min()
        .unwrap();
    println!("{} x {} = {}", bus_id, wait_time, bus_id * wait_time);

    let id_index_pairs = buses_raw
        .split(",")
        .enumerate()
        .filter(|(_, id)| *id != "x")
        .map(|(i, id)| (p_u64(id), i as u64))
        .collect::<Vec<_>>();

    let mut start = 0;
    let (mut step, _) = id_index_pairs[0];
    for i in 1..id_index_pairs.len() {
        start = find_valid_schedule_start(start, step, &id_index_pairs[0..=i]);
        step *= id_index_pairs[i].0;
    }
    println!("{}", start);

    // let (largest_id, offset) = id_index_pairs.iter().max().unwrap();
    // let mut current = min_pt2 + (largest_id - (min_pt2 % largest_id));
    // let mut values = Vec::new();
    // for _ in 0..3 {
    //     while id_index_pairs
    //         .iter()
    //         .any(|(id, index)| (current - offset + index) % id != 0)
    //     {
    //         current += largest_id;
    //         // if &(current % 100_000_000_000) < largest_id {
    //         //     println!("{}", current);
    //         // }
    //         // if current > 1067000 {
    //         //     println!("{}", current);
    //         // }
    //         // if current > 1069000 {
    //         //     panic!("Stop: {}", current);
    //         // }
    //     }
    //     values.push(current - offset);
    //     println!("{}", current - offset);
    //     current += largest_id;
    // }
    // println!(
    //     "{:?} -> {} {}",
    //     values,
    //     values[1] - values[0],
    //     values[2] - values[1]
    // )
}

fn find_valid_schedule_start(start: u64, step: u64, id_index_pairs: &[(u64, u64)]) -> u64 {
    let mut current = start;
    while id_index_pairs
        .iter()
        .any(|(id, index)| (current + index) % id != 0)
    {
        current += step;
        // if &(current % 100_000_000_000) < largest_id {
        //     println!("{}", current);
        // }
        // if current > 1067000 {
        //     println!("{}", current);
        // }
        // if current > 1069000 {
        //     panic!("Stop: {}", current);
        // }
    }
    current
}
