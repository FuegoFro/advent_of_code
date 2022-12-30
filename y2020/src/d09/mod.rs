use std::collections::VecDeque;

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    // let buffer = 5;
    let input = include_str!("actual_input.txt").trim();
    let buffer = 25;

    let numbers = input
        .split('\n')
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let weak_number = find_weak_number(buffer, &numbers);
    println!("Weak number: {}", weak_number);
    let subset = find_subset(weak_number, &numbers);

    println!(
        "Added subset: {}",
        subset.iter().min().unwrap() + subset.iter().max().unwrap()
    );
}

fn find_weak_number(buffer: usize, numbers: &[i64]) -> i64 {
    let mut numbers = numbers.iter();
    let mut valid_previous = (0..buffer)
        .map(|_| numbers.next().unwrap())
        .collect::<VecDeque<_>>();
    for number in numbers {
        if !valid_previous
            .iter()
            .any(|valid| valid_previous.contains(&&(number - *valid)))
        {
            return *number;
        }
        valid_previous.pop_front();
        valid_previous.push_back(number);
    }
    panic!("Could not find weak number!");
}

fn find_subset(target: i64, numbers: &Vec<i64>) -> &[i64] {
    let mut low = 0;
    let mut high = 0;
    let mut total = numbers[0];
    while low < numbers.len() && high < numbers.len() && total != target {
        if total > target {
            assert!(low <= high);
            total -= numbers[low];
            low += 1;
        } else {
            high += 1;
            total += numbers[high];
        }
    }
    if total == target {
        &numbers[low..=high]
    } else {
        panic!("Unable to find subset!")
    }
}
