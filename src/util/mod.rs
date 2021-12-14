pub mod point;
pub mod point2;
pub mod grid;
pub mod iter_helpers;

#[allow(dead_code)]
pub fn p_u32(s: &str) -> u32 {
    s.parse().expect(s)
}

pub fn p_u32c(c: char) -> u32 {
    c.to_digit(10).expect(&format!("-->{}<--", c))
}

pub fn p_u64(s: &str) -> u64 {
    s.parse().expect(s)
}

#[allow(dead_code)]
pub fn p_i32(s: &str) -> i32 {
    s.parse().expect(s)
}

#[allow(dead_code)]
pub fn p_i64(s: &str) -> i64 {
    s.parse().expect(s)
}

pub fn split_once<'a>(s: &'a str, delim: &str) -> (&'a str, &'a str) {
    let mut split = s.splitn(2, delim);
    (split.next().unwrap(), split.next().unwrap())
}
