use crate::util::grid::Grid;
use crate::util::p_i32;
use crate::util::point::{get_bounding_box, Point};
use itertools::Itertools;
use std::cmp::min;
use std::collections::HashSet;
use tuple::Map;

enum FoldAxis {
    X,
    Y,
}

struct Fold {
    axis: FoldAxis,
    distance: i32,
}

impl Fold {
    fn from_str(s: &str) -> Self {
        assert!(s.starts_with("fold along "));
        let (first_half, distance) = s.split_once("=").unwrap();
        Fold {
            axis: match first_half.chars().last().unwrap() {
                'x' => FoldAxis::X,
                'y' => FoldAxis::Y,
                _ => panic!(),
            },
            distance: p_i32(distance),
        }
    }

    fn update(&self, mut point: Point) -> Point {
        let value = match self.axis {
            FoldAxis::X => point.x,
            FoldAxis::Y => point.y,
        };
        assert_ne!(value, self.distance);
        if value > self.distance {
            let new_value = self.distance - (value - self.distance);
            match self.axis {
                FoldAxis::X => point.x = new_value,
                FoldAxis::Y => point.y = new_value,
            }
        }
        point
    }
}

fn fold_points(mut points: HashSet<Point>, folds: &Vec<Fold>) -> HashSet<Point> {
    // print_points(&points);
    for fold in folds {
        points = fold_once(points, fold);
    }
    points
}

fn fold_once(mut points: HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    points = points.into_iter().map(|p| fold.update(p)).collect();
    // Rectify points
    let (mut min_point, _) = get_bounding_box(points.iter());
    min_point.x = min(min_point.x, 0);
    min_point.y = min(min_point.y, 0);
    points.into_iter().map(|p| p - min_point).collect()
}

fn print_points(points: &HashSet<Point>) {
    let (_, max_point) = get_bounding_box(points.iter());
    let storage = (0..=max_point.y)
        .map(|y| {
            (0..=max_point.x)
                .map(|x| {
                    let p = Point::new(x, y);
                    if points.contains(&p) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect_vec()
        })
        .collect_vec();
    dbg!(Grid::from_storage(storage));
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace("\r", "");

    let (points_raw, folds_raw) = input.split_once("\n\n").unwrap();
    let points = points_raw
        .split("\n")
        .map(|line| line.split_once(",").unwrap().map(p_i32))
        .map(|(x, y)| Point::new(x, y))
        .collect::<HashSet<_>>();
    let folds = folds_raw.split("\n").map(Fold::from_str).collect_vec();

    let folded = fold_once(points.clone(), &folds[0]);
    // print_points(&folded);
    println!("Part 1: {}", folded.len());
    println!("Part 2:");
    print_points(&fold_points(points, &folds));
}
