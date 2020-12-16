use anyhow::Result;
use regex::Regex;

#[derive(Debug)]
struct Password {
    num_a: usize,
    num_b: usize,
    char: char,
    content: String,
}

impl Password {
    fn from_packed(packed: &str) -> Result<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<num_a>\d+)-(?P<num_b>\d+) (?P<char>[a-z]): (?P<content>[a-z]+)$")
                    .unwrap();
        }
        let caps = RE.captures(packed).expect(packed);
        Ok(Password {
            num_a: caps.name("num_a").unwrap().as_str().parse::<usize>()?,
            num_b: caps.name("num_b").unwrap().as_str().parse::<usize>()?,
            char: caps.name("char").unwrap().as_str().chars().next().unwrap(),
            content: caps.name("content").unwrap().as_str().to_owned(),
        })
    }

    #[allow(dead_code)]
    fn is_valid_old(&self) -> bool {
        let num_char = self.content.chars().filter(|c| *c == self.char).count();
        self.num_a <= num_char && num_char <= self.num_b
    }

    fn is_valid_new(&self) -> bool {
        let has_char_at_a = self.content.chars().nth(self.num_a - 1).unwrap() == self.char;
        let has_char_at_b = self.content.chars().nth(self.num_b - 1).unwrap() == self.char;

        has_char_at_a ^ has_char_at_b
    }
}

pub fn main() {
    // let passwords = include_str!("example_passwords.txt").trim();
    let passwords = include_str!("target_passwords.txt").trim();

    let num_valid = passwords
        .split("\n")
        .map(|line| Password::from_packed(line).unwrap())
        .filter(|password| password.is_valid_new())
        .count();
    println!("{}", num_valid);
}
