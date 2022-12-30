use itertools::{Itertools, MinMaxResult};

pub mod grid;
pub mod iter_helpers;
pub mod point;
pub mod point2;
pub mod point3;

#[allow(dead_code)]
pub fn p_u32(s: &str) -> u32 {
    s.parse().expect(s)
}

#[allow(dead_code)]
pub fn p_usize(s: &str) -> usize {
    s.parse().expect(s)
}

pub fn p_u32c(c: char) -> u32 {
    c.to_digit(10).unwrap_or_else(|| panic!("-->{}<--", c))
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

pub fn min_max<N: PartialOrd + Copy>(vals: Vec<N>) -> (N, N) {
    match vals.into_iter().minmax() {
        MinMaxResult::NoElements => panic!("Expected some elements"),
        MinMaxResult::OneElement(e) => (e, e),
        MinMaxResult::MinMax(l, h) => (l, h),
    }
}
