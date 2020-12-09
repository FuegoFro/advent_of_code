use std::collections::{HashMap, HashSet};
use std::ops;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl_op_ex!(+ |a: &Point, b: &Point| -> Point { Point { x: a.x + b.x, y: a.y + b.y }});
impl_op!(+= |a: &mut Point, b: Point| { *a = &*a + b });
impl_op!(+= |a: &mut Point, b: &Point| { *a = &*a + b });

impl Point {
    fn l1_dist(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn visited_locations(path: &str) -> HashMap<Point, u32> {
    let mut locations = HashMap::new();
    let mut current = Point { x: 0, y: 0 };
    let mut num_steps = 0;
    for (direction, count) in path.split(",").map(|p| p.split_at(1)) {
        let count = count.parse::<i32>().unwrap();
        let direction = match direction {
            "L" => Point { x: -1, y: 0 },
            "R" => Point { x: 1, y: 0 },
            "U" => Point { x: 0, y: 1 },
            "D" => Point { x: 0, y: -1 },
            _ => panic!("Unknown direction {}", direction),
        };
        for _ in 0..count {
            current += &direction;
            num_steps += 1;
            locations.entry(current).or_insert(num_steps);
        }
    }
    locations
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let mut split = input.split("\n").map(visited_locations);
    let a = split.next().unwrap();
    let b = split.next().unwrap();
    assert!(split.next().is_none());

    let all_intersections = &a.keys().collect::<HashSet<_>>() & &b.keys().collect();
    closest_l1(&all_intersections);
    closest_path(&a, &b, &all_intersections);
}

fn closest_l1(all_intersections: &HashSet<&Point>) {
    let closest_intersection = all_intersections
        .iter()
        .fold(None, |best: Option<&Point>, current| {
            best.map(|best| {
                if current.l1_dist() < best.l1_dist() {
                    current
                } else {
                    best
                }
            })
            .or(Some(current))
        })
        .unwrap();
    println!(
        "pt1 point = {:?}, dist = {}",
        closest_intersection,
        closest_intersection.l1_dist()
    );
}

fn closest_path(
    a: &HashMap<Point, u32>,
    b: &HashMap<Point, u32>,
    all_intersections: &HashSet<&Point>,
) {
    let path_len = |p: &Point| a[p] + b[p];

    let closest_intersection = all_intersections
        .iter()
        .fold(None, |best: Option<&Point>, current| {
            best.map(|best| {
                if path_len(current) < path_len(best) {
                    current
                } else {
                    best
                }
            })
            .or(Some(current))
        })
        .unwrap();
    println!(
        "pt2 point = {:?} dist = {}",
        closest_intersection,
        path_len(closest_intersection)
    );
}
