use itertools::Itertools;
use regex::{Captures, Regex};

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let pt1 = input
        .lines()
        .map(|l| {
            let chars = l.chars().filter_map(|c| c.to_digit(10)).collect_vec();
            chars.first().unwrap() * 10 + chars.last().unwrap()
        })
        .sum::<u32>();

    println!("Part 1: {}", pt1);

    let digits_re_str = r"one|two|three|four|five|six|seven|eight|nine|zero".to_owned();
    let regex = Regex::new(&format!(r"[0-9]|{}", digits_re_str)).unwrap();
    let regex_rev = Regex::new(&format!(r"[0-9]|{}", digits_re_str.rev())).unwrap();
    let pt2 = input
        .lines()
        .map(|l| {
            let first = regex.captures_iter(l).next();
            let l_rev = l.rev();
            let last = regex_rev.captures_iter(&l_rev).next();
            let num = to_digit(first.as_ref(), false) * 10 + to_digit(last.as_ref(), true);
            println!("{} {}", num, l);
            num
        })
        .sum::<u32>();

    println!("Part 2: {}", pt2);
}

fn to_digit(c: Option<&Captures>, rev: bool) -> u32 {
    let mut s = c.unwrap().get(0).unwrap().as_str().to_owned();
    if rev {
        s = s.rev()
    }
    if s.len() == 1 {
        s.parse().unwrap()
    } else {
        match s.as_str() {
            "zero" => 0,
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => panic!("Unknown digit {}", s),
        }
    }
}

trait StringRev<T>
where
    T: AsRef<str>,
{
    fn rev(self) -> String;
}

impl<T> StringRev<T> for T
where
    T: AsRef<str>,
{
    fn rev(self) -> String {
        self.as_ref().chars().rev().collect()
    }
}
