fn from_snafu(s: &str) -> i64 {
    let mut n = 0;
    for (exponent, digit) in s.chars().rev().enumerate() {
        let offset = match digit {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '=' => -2,
            '-' => -1,
            _ => panic!("Unknown digit {}", digit),
        };
        n += 5i64.pow(exponent as u32) * offset
    }
    n
}

fn to_snafu(mut n: i64) -> String {
    if n < 0 {
        println!("Warning, using negative number! {}", n);
    }
    let mut reversed = String::new();
    while n != 0 {
        let (next_offset, next_digit) = match n % 5 {
            0 => (0, '0'),
            1 => (1, '1'),
            2 => (2, '2'),
            3 => (-2, '='),
            4 => (-1, '-'),
            _ => unreachable!(),
        };
        reversed.push(next_digit);
        n = (n - next_offset) / 5;
    }
    reversed.chars().rev().collect()
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let total = input.split('\n').map(from_snafu).sum();

    println!("Part 1: {}", to_snafu(total));
    // println!("Part 2: {}", 2);
}
