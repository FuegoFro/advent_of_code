pub mod point;

#[allow(dead_code)]
pub fn p_u32(s: &str) -> u32 {
    s.parse().expect(s)
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
