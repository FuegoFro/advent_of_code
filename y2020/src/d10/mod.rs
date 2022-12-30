pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let mut nums = input
        .split('\n')
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    nums.sort();
    let target = nums.last().unwrap() + 3;
    nums.push(target);
    let (ones, threes, _) = nums
        .iter()
        .fold((0, 0, 0), |(mut ones, mut threes, prev), current| {
            match current - prev {
                1 => ones += 1,
                3 => threes += 1,
                _ => panic!("Unknown diff between {} and {}", prev, current),
            };
            (ones, threes, *current)
        });

    println!("{}, {} -> {}", ones, threes, ones * threes);

    let mut ways_to_get_joltage = vec![0_usize; target + 1];
    ways_to_get_joltage[0] = 1;
    for adapter in &nums {
        let previous: usize = (1..=3)
            .filter(|offset| offset <= adapter)
            .map(|offset| ways_to_get_joltage[adapter - offset])
            .sum();
        ways_to_get_joltage[*adapter] = previous;
    }

    println!("{}", ways_to_get_joltage.last().unwrap());
}

/*
[]
1:
(0) (3)

[2]
1:
(0) 2 (5)

[2,3]
2:
(0) 2 3 (6)
(0) 3 (6)

[2,3,5]
3:
(0) 2 3 5 (8)
(0) 2 5 (8)
(0) 3 5 (8)

[2,3,5,6]
5:
(0) 2 3 5 6 (9)
(0) 2 3 6 (9)
(0) 2 5 6 (9)
(0) 3 5 6 (9)
(0) 3 6 (9)




[]
1:
(0) (3)

[2]
1:
(0) 2 (5)

[2,3]
2:
(0) 2 3 (6)
(0) 3 (6)

[2,3,6]
2:
(0) 2 3 6 (9)
(0) 3 6 (9)

[2,3,6,8]
2:
(0) 2 3 6 8 (11)
(0) 3 6 8 (11)

[2,3,6,8,9]
4:
(0) 2 3 6 8 9 (12)
(0) 2 3 6 9 (12)
(0) 3 6 8 9 (12)
(0) 3 6 9 (12)

 */
