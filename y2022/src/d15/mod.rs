#![allow(clippy::needless_question_mark)]
use itertools::Itertools;
use recap::Recap;
use serde::Deserialize;
use std::collections::HashSet;
use util::point2::{PointS};

#[derive(Deserialize, Recap)]
#[recap(
    regex = r#"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)"#
)]
struct SensorRaw {
    sensor_x: i32,
    sensor_y: i32,
    beacon_x: i32,
    beacon_y: i32,
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    // let pt1_target_row = 10;
    // let pt2_search_space = 20;
    let input = include_str!("actual_input.txt").trim().replace('\r', "");
    let pt1_target_row = 2000000;
    let pt2_search_space = 4000000;

    let sensor_beacon_pairs = input
        .split('\n')
        .map(|l| l.parse::<SensorRaw>().unwrap())
        .map(|s| {
            (
                PointS::new(s.sensor_x, s.sensor_y),
                PointS::new(s.beacon_x, s.beacon_y),
            )
        })
        .collect_vec();

    let beacons = sensor_beacon_pairs
        .iter()
        .map(|(_, b)| *b)
        .collect::<HashSet<_>>();

    let sensors_and_distances = sensor_beacon_pairs
        .iter()
        .map(|(s, b)| {
            let distance = (s - b).l1_dist();
            (*s, distance)
        })
        .collect_vec();

    let max_distance = sensors_and_distances.iter().map(|(_, d)| *d).max().unwrap();
    let (min_sensor, max_sensor) =
        PointS::get_bounding_box(sensors_and_distances.iter().map(|(s, _)| s));

    let mut no_beacon = 0;
    for x in min_sensor.x - max_distance..=max_sensor.x + max_distance {
        let point = PointS::new(x, pt1_target_row);
        if !beacons.contains(&point)
            && sensors_and_distances
                .iter()
                .any(|(s, d)| (point - *s).l1_dist() <= *d)
        {
            no_beacon += 1;
        }
    }

    println!("Part 1: {}", no_beacon);

    let mut missing_beacon = None;
    // let mut point = PointS::ORIGIN;
    for y in 0..=pt2_search_space {
        // point.y = y;
        // if y % 1 == 0 {
        // println!("y={}", y);
        // }
        // point.x = 0;
        let mut x = 0;
        while x <= pt2_search_space {
            let next_x = sensors_and_distances
                .iter()
                .filter_map(|(sensor, distance)| {
                    let y_dist = (sensor.y - y).abs();
                    let max_x_dist = distance - y_dist;
                    if (sensor.x - x).abs() <= max_x_dist {
                        Some(sensor.x + max_x_dist + 1)
                    } else {
                        None
                    }
                })
                .max();
            if let Some(next_x) = next_x {
                x = next_x;
            } else {
                missing_beacon = Some(PointS::new(x, y));
                break;
            }
        }
    }

    let missing_beacon = dbg!(missing_beacon.unwrap());
    let tuning_frequency = missing_beacon.x as i64 * 4000000 + missing_beacon.y as i64;

    println!("Part 2: {}", tuning_frequency);
}
