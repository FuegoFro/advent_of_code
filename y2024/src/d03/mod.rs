use regex::Regex;
use util::p_u32;

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let p1 = mul_regex
        .captures_iter(&input)
        .map(|c| p_u32(c.get(1).unwrap().as_str()) * p_u32(c.get(2).unwrap().as_str()))
        .sum::<u32>();

    println!("Part 1: {}", p1);

    let mul_do_dont_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let p2 = mul_do_dont_regex
        .captures_iter(&input)
        .fold((true, 0), |(enabled, sum), c| {
            let full_match = c.get(0).unwrap().as_str();
            if full_match.starts_with("don't") {
                (false, sum)
            } else if full_match.starts_with("do") {
                (true, sum)
            } else if enabled {
                (
                    enabled,
                    sum + p_u32(c.get(1).unwrap().as_str()) * p_u32(c.get(2).unwrap().as_str()),
                )
            } else {
                (enabled, sum)
            }
        })
        .1;
    println!("Part 2: {}", p2);
}
