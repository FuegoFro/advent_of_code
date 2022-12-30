use std::collections::HashSet;

use itertools::Itertools;

use util::p_i32;
use util::point3::{Delta3, Point3};

struct Rotation {
    looking: Delta3,
    up: Delta3,
}

impl Rotation {
    const NOOP: Rotation = Rotation::new(Delta3::X_POS, Delta3::Z_POS);

    const ALL_ALIGNED: [Rotation; 24] = [
        Rotation::new(Delta3::X_POS, Delta3::Y_POS),
        Rotation::new(Delta3::X_POS, Delta3::Y_NEG),
        Rotation::new(Delta3::X_POS, Delta3::Z_POS),
        Rotation::new(Delta3::X_POS, Delta3::Z_NEG),
        Rotation::new(Delta3::X_NEG, Delta3::Y_POS),
        Rotation::new(Delta3::X_NEG, Delta3::Y_NEG),
        Rotation::new(Delta3::X_NEG, Delta3::Z_POS),
        Rotation::new(Delta3::X_NEG, Delta3::Z_NEG),
        Rotation::new(Delta3::Y_POS, Delta3::X_POS),
        Rotation::new(Delta3::Y_POS, Delta3::X_NEG),
        Rotation::new(Delta3::Y_POS, Delta3::Z_POS),
        Rotation::new(Delta3::Y_POS, Delta3::Z_NEG),
        Rotation::new(Delta3::Y_NEG, Delta3::X_POS),
        Rotation::new(Delta3::Y_NEG, Delta3::X_NEG),
        Rotation::new(Delta3::Y_NEG, Delta3::Z_POS),
        Rotation::new(Delta3::Y_NEG, Delta3::Z_NEG),
        Rotation::new(Delta3::Z_POS, Delta3::X_POS),
        Rotation::new(Delta3::Z_POS, Delta3::X_NEG),
        Rotation::new(Delta3::Z_POS, Delta3::Y_POS),
        Rotation::new(Delta3::Z_POS, Delta3::Y_NEG),
        Rotation::new(Delta3::Z_NEG, Delta3::X_POS),
        Rotation::new(Delta3::Z_NEG, Delta3::X_NEG),
        Rotation::new(Delta3::Z_NEG, Delta3::Y_POS),
        Rotation::new(Delta3::Z_NEG, Delta3::Y_NEG),
    ];

    const fn new(looking: Delta3, up: Delta3) -> Self {
        Rotation { looking, up }
    }

    fn rotate(&self, a: i32, b: i32, c: i32) -> (i32, i32, i32) {
        // Working from https://stackoverflow.com/a/6802424, ignoring translation
        let orig = Delta3::new(a, b, c);
        let x_axis = self.looking;
        let z_axis = self.up;
        let y_axis = z_axis.cross(&x_axis);
        // TODO - Unclear if this matrix mult direction is correct.
        (orig.dot(&x_axis), orig.dot(&y_axis), orig.dot(&z_axis))
        // let xs = Delta3::new(x_axis.dx, y_axis.dx, z_axis.dx);
        // let ys = Delta3::new(x_axis.dy, y_axis.dy, z_axis.dy);
        // let zs = Delta3::new(x_axis.dz, y_axis.dz, z_axis.dz);
        // (orig.dot(&xs), orig.dot(&ys), orig.dot(&zs))
    }
}

// TODO(refactor) - Move to util crate
trait DeltaExtension {
    fn cross(&self, other: &Delta3) -> Delta3;
    fn dot(&self, other: &Delta3) -> i32;
    // fn rotate(&self, rotation: &Rotation) -> Self;
}

impl DeltaExtension for Delta3 {
    fn cross(&self, other: &Delta3) -> Delta3 {
        Delta3::new(
            self.dy * other.dz - self.dz * other.dy,
            self.dz * other.dx - self.dx * other.dz,
            self.dx * other.dy - self.dy * other.dx,
        )
    }
    fn dot(&self, other: &Delta3) -> i32 {
        self.dx * other.dx + self.dy * other.dy + self.dz * other.dz
    }
    // fn rotate(&self, rotation: &Rotation) -> Self {
    //     let (dx, dy, dz) = rotation.rotate(self.dx, self.dy, self.dz);
    //     Self { dx, dy, dz }
    // }
}

trait PointExtension {
    fn rotate(&self, rotation: &Rotation) -> Point3;
}
impl PointExtension for Point3 {
    fn rotate(&self, rotation: &Rotation) -> Point3 {
        let (x, y, z) = rotation.rotate(self.x, self.y, self.z);
        Self { x, y, z }
    }
}

type PointDeltas = Vec<(Point3, HashSet<Delta3>)>;
type PointMappings = Vec<(Point3, Point3)>;

// We need 12 points, but we insert both directions of the delta so we double it. This is used
// both to determine if two individual points correlate and if we've correlated enough points
// for a valid rotation.
const MIN_CORRELATION_COUNT: usize = 12;

fn build_deltas<'a, T>(points: T, rotation: &Rotation) -> PointDeltas
where
    T: Iterator<Item = &'a Point3> + Clone,
{
    // Put both directions for all pairs of points in the map
    points
        .map(|p| p.rotate(rotation))
        .tuple_combinations::<(_, _)>()
        // Treat each point as the primary point
        .flat_map(|(a, b)| [(a, b), (b, a)].into_iter())
        // Include both directions of the delta to not worry about ordering. Means we need to look
        // for twice as many overlapping deltas.
        // Order them so the dx is positive, so we have consistent ordering.
        .map(|(a, b)| (a, if a.x > b.x { a - b } else { b - a }))
        // Group them by primary point
        .into_group_map()
        // Convert to pairs with deltas in a hash set.
        .into_iter()
        .map(|(point, deltas)| (point, deltas.into_iter().collect()))
        .collect_vec()
}

// (1317, -1543, 960)

fn canonical_points_from_deltas(
    canonical_point_deltas: &PointDeltas,
    points: &[Point3],
) -> Option<(Vec<Point3>, Delta3)> {
    for rotation in Rotation::ALL_ALIGNED.iter() {
        let other_point_deltas = build_deltas(points.iter(), rotation);
        // print_deltas("other", &other_point_deltas);
        let point_mappings = correlate_points(canonical_point_deltas, &other_point_deltas);
        if point_mappings.len() >= MIN_CORRELATION_COUNT {
            let translation = calculate_translation(point_mappings);
            let translated_points = other_point_deltas
                .iter()
                .map(|(p, _)| p + translation)
                .collect_vec();
            return Some((translated_points, translation));
        }
    }
    None
}

// /// Rotates the points and deltas
// fn rotate_point_deltas(point_deltas: &PointDeltas, rotation: &Rotation) -> PointDeltas {
//     point_deltas
//         .iter()
//         .map(|(point, deltas)| {
//             (
//                 point.rotate(rotation),
//                 deltas.iter().map(|delta| delta.rotate(rotation)).collect(),
//             )
//         })
//         .collect_vec()
// }

/// Find mappings from canonical to other where they share enough deltas
fn correlate_points(
    canonical_point_deltas: &PointDeltas,
    other_point_deltas: &PointDeltas,
) -> PointMappings {
    let mut mappings = Vec::new();
    for (canonical_point, canonical_deltas) in canonical_point_deltas.iter() {
        for (other_point, other_deltas) in other_point_deltas.iter() {
            // Reduce by one since we don't look at the current point.
            if canonical_deltas.intersection(other_deltas).count() >= MIN_CORRELATION_COUNT - 1 {
                mappings.push((*canonical_point, *other_point))
            }
        }
    }
    mappings
}

fn calculate_translation(point_mappings: PointMappings) -> Delta3 {
    let all_deltas = point_mappings
        .iter()
        .map(|(a, b)| a - b)
        .unique()
        .collect_vec();
    // dbg!(&all_deltas);
    assert_eq!(all_deltas.len(), 1);
    all_deltas[0]
}

#[allow(dead_code)]
fn print_deltas(name: &str, point_deltas: &PointDeltas) {
    println!("vvvvv {} vvvvv", name);
    for (p, deltas) in point_deltas {
        let deltas_str = deltas
            .iter()
            .map(|d| format!("({}, {}, {})", d.dx, d.dy, d.dz))
            .join(", ");
        println!(
            "Point3=({}, {}, {})   --->   {{{}}}",
            p.x, p.y, p.z, deltas_str
        );
    }
    println!("^^^^^ {} ^^^^^", name);
    println!();
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let sensor_values = input
        .split("\n\n")
        .map(|block| {
            block
                .split('\n')
                .skip(1)
                .map(|line| {
                    line.split(',')
                        .map(p_i32)
                        .collect_tuple::<(_, _, _)>()
                        .map(|(x, y, z)| Point3::new(x, y, z))
                        .unwrap()
                })
                .collect_vec()
        })
        .collect_vec();

    // Build a delta-map of sensor 0
    // For each remaining sensor
    //      Build a delta map
    //      For each rotation of the delta map
    //          Find the number of overlaps w/ delta-map 0
    //              Each delta that matches, map those points from current to 0
    //          If the number is 12, treat that as the canonical rotation
    //      Rotate the sensor to 0's coordinate space
    //      If we found enough points
    //          Add the points to the global set of points
    //      Else
    //          Put the sensor back on the queue
    //
    // NOTE! We re-calculate all the rotated point-deltas each time we try a given sensor, this
    // is super inefficient. But it's fast enough to eventually get an answer (in a minute or so)

    // Seed our set of all points with the first sensor's values.
    let (all_points, rest) = sensor_values.split_first().unwrap();
    let mut all_points = all_points.iter().cloned().collect::<HashSet<_>>();
    let mut remaining_scanners = rest.iter().cloned().collect_vec();
    let mut translations = vec![Delta3::IDENT];
    let mut did_work = true;
    while !remaining_scanners.is_empty() && did_work {
        did_work = false;
        let mut skipped = Vec::new();
        for sensor_points in remaining_scanners {
            let canonical_point_deltas = build_deltas(all_points.iter(), &Rotation::NOOP);
            // let other_point_deltas = build_deltas(sensor_points.iter());
            // print_deltas("canonical", &canonical_point_deltas);
            // print_deltas("other", &other_point_deltas);
            match canonical_points_from_deltas(&canonical_point_deltas, &sensor_points) {
                None => skipped.push(sensor_points),
                Some((new_canonical_points, translation)) => {
                    did_work = true;
                    all_points.extend(new_canonical_points);
                    translations.push(translation);
                }
            }
        }
        remaining_scanners = skipped;
    }
    assert!(remaining_scanners.is_empty());

    println!("Part 1: {}", all_points.len());

    let max_sensor_distance = translations
        .iter()
        // Convert deltas to points
        .map(|t| Point3::ORIGIN + t)
        // For each pair, calculate the manhattan distance
        .tuple_combinations::<(_, _)>()
        .map(|(a, b)| (a - b).l1_dist())
        .max()
        .unwrap();
    println!("Part 2: {}", max_sensor_distance);
}
