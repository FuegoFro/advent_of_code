use std::collections::HashMap;

fn find_pos(input: &str, distinct_count: usize) -> usize {
    let offset = distinct_count - 1;
    let mut seen_counts = HashMap::<_, u32>::new();
    let mut num_dupes = 0;
    for (i, letter) in input.chars().enumerate() {
        let entry = seen_counts.entry(letter).or_default();
        *entry += 1;
        if *entry == 2 {
            num_dupes += 1;
        }

        if i < offset {
            continue;
        }

        if num_dupes == 0 {
            return i + 1;
        }

        let old_letter = input.chars().nth(i - offset).unwrap();
        let entry = seen_counts.entry(old_letter).or_default();
        *entry -= 1;
        if *entry == 1 {
            num_dupes -= 1;
        }
    }

    0
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let pos1 = find_pos(&input, 4);
    println!("Part 1: {}", pos1);

    let pos2 = find_pos(&input, 14);
    println!("Part 2: {}", pos2);
}
