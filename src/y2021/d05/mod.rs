use crate::util::p_i32;
use crate::util::point::Point;
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::repeat;

fn str_to_point(s: &str) -> Point {
    let (x, y) = s.split_once(",").unwrap();
    Point::new(p_i32(x), p_i32(y))
}

fn make_point_iter(start: i32, end: i32) -> Box<dyn Iterator<Item = i32>> {
    if start == end {
        // println!("    Repeating {}", start);
        Box::new(repeat(start))
    } else if start < end {
        // println!("    Range {}..={}", start, end);
        Box::new(start..=end)
    } else {
        // println!("    Range {}..={}", end, start);
        Box::new((end..=start).rev())
    }
}

fn get_num_overlapped(starts_and_ends: &Vec<(Point, Point)>, ignore_diagonal: bool) -> usize {
    let mut seen_points = HashMap::new();
    for (start, end) in starts_and_ends.iter() {
        // println!("Have points {:?} -> {:?}", start, end);
        if start.x != end.x && start.y != end.y && ignore_diagonal {
            // println!("BAILING - Diagonal");
            continue;
        }
        for (line_x, line_y) in make_point_iter(start.x, end.x).zip(make_point_iter(start.y, end.y))
        {
            let line_point = Point::new(line_x, line_y);
            // println!("Inserting point {:?}", line_point);
            *seen_points.entry(line_point).or_insert(0u32) += 1;
        }
    }
    seen_points.values().filter(|v| **v >= 2).count()
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace("\r", "");

    let starts_and_ends = input
        .split("\n")
        .map(|line| {
            let (start_raw, end_raw) = line.split_once(" -> ").unwrap();
            (str_to_point(start_raw), str_to_point(end_raw))
        })
        .collect_vec();

    println!("Part 1: {}", get_num_overlapped(&starts_and_ends, true));
    println!("Part 2: {}", get_num_overlapped(&starts_and_ends, false));
}
