use crate::util::point::Point;
use itertools::Itertools;
use num::traits::FloatConst;
use num::Integer;
use std::collections::HashSet;

struct Asteroids {
    map: Vec<Vec<bool>>,
    positions: Vec<Point>,
}

impl Asteroids {
    fn from_packed(packed: &str) -> Self {
        let map = packed
            .lines()
            .map(|l| l.chars().map(|c| c == '#').collect_vec())
            .collect_vec();
        let positions = map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, v)| **v)
                    .map(move |(x, _)| Point::new(x as i32, y as i32))
            })
            .collect_vec();
        Asteroids { map, positions }
    }

    fn has_asteroid(&self, point: Point) -> bool {
        self.map[point.y as usize][point.x as usize]
    }

    fn iter(&self) -> impl Iterator<Item = &Point> {
        self.positions.iter()
    }

    fn visible_from(&self, curr: &Point, exclude: &HashSet<&Point>) -> Vec<&Point> {
        self.iter()
            .filter(move |other| !exclude.contains(other))
            .filter(move |other| *other != curr)
            .filter(move |other| {
                let delta = *other - curr;
                let gcd = delta.x.gcd(&delta.y);
                let delta = delta / gcd;
                !(1..gcd).any(|mult| {
                    let test = delta * mult + curr;
                    self.has_asteroid(test) && !exclude.contains(&test)
                })
            })
            .collect_vec()
    }
}

fn norm_angle(p: &Point) -> f64 {
    let angle = (p.atan2() / f64::PI() + 1.5) % 2.0;
    if angle == 0.0 {
        2.0
    } else {
        angle
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let asteroids = Asteroids::from_packed(input);

    let mut exclude = HashSet::new();
    let (most, station) = asteroids
        .iter()
        .map(|curr| (asteroids.visible_from(curr, &exclude).len(), curr))
        .max_by(|(v_a, _), (v_b, _)| v_a.cmp(v_b))
        .unwrap();

    println!("{:?}", most);
    println!("{:?}", station);
    // for (x, y) in [
    //     (0, 1),
    //     (1, 1),
    //     (1, 0),
    //     (1, -1),
    //     (0, -1),
    //     (-1, -1),
    //     (-1, 0),
    //     (-1, 1),
    // ]
    // .iter()
    // {
    //     let p = Point::new(*x, *y);
    //     let atan = p.atan2();
    //     println!(
    //         "({}, {}) -> {:.2}, {:.2}, {:.2}",
    //         x,
    //         y,
    //         atan / f64::PI(),
    //         (atan / f64::PI() + 1.5),
    //         norm_angle(&p),
    //     );
    // }

    let mut destroyed = Vec::new();
    while destroyed.len() < 200 && destroyed.len() < asteroids.positions.len() - 1 {
        println!("!!!!!!! WAVE");
        let mut next_wave = asteroids
            .visible_from(&station, &exclude)
            .into_iter()
            .map(|p| {
                // Convert to angle from vertical
                let mut direction = p - station;
                direction.y = -direction.y;
                let angle = norm_angle(&direction);
                // println!("{:?} -> {:?} ({})", p, direction, angle);
                (angle, p)
            })
            // Sort in reverse
            .sorted_by(|(angle_a, _), (angle_b, _)| angle_b.partial_cmp(angle_a).unwrap())
            .map(|(_, p)| p)
            .collect_vec();
        next_wave.iter().for_each(|p| {
            exclude.insert(p);
        });
        destroyed.append(&mut next_wave);
    }
    println!("{:?}", &destroyed[..3]);
    println!("{:?}", destroyed[199]);
}
