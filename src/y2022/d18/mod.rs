use crate::util::p_i32;
use crate::util::point3::{BoundingBox, Delta3, Point3};
use itertools::Itertools;
use std::collections::HashSet;
use std::ops::Sub;

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let points = input
        .split('\n')
        .map(|l| {
            let (x, y, z) = l.split(',').map(p_i32).collect_tuple().unwrap();
            Point3::new(x, y, z)
        })
        .collect::<HashSet<_>>();

    let num_exposed_sides: usize = points
        .iter()
        .map(|point| {
            Delta3::NEIGHBORS_6
                .iter()
                .filter(|offset| !points.contains(&(point + *offset)))
                .count()
        })
        .sum();

    println!("Part 1: {}", num_exposed_sides);

    let bb = Point3::get_bounding_box(points.iter());
    // Expand it by 1
    let bb = BoundingBox::new(
        bb.start + Delta3::X_NEG + Delta3::Y_NEG + Delta3::Z_NEG,
        bb.end + (Delta3::X_POS + Delta3::Y_POS + Delta3::Z_POS) * 2,
    );

    let mut seen_edges: HashSet<(Point3, Point3)> = HashSet::new();
    let mut frontier = vec![bb.start];
    let mut visited: HashSet<Point3> = HashSet::new();
    while let Some(node) = frontier.pop() {
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);

        for offset in Delta3::NEIGHBORS_6.iter() {
            let neighbor = node + offset;
            if !bb.contains(&neighbor) {
                continue;
            }
            if points.contains(&neighbor) {
                seen_edges.insert(
                    [node, neighbor]
                        .into_iter()
                        .sorted()
                        .collect_tuple()
                        .unwrap(),
                );
            } else {
                frontier.push(neighbor);
            }
        }
    }

    println!("Part 2: {}", seen_edges.len());
}
