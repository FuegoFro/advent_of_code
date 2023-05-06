use std::collections::{HashMap, HashSet};
use util::grid::Grid;
use util::point2::{Delta, PointS, Rotation};

const DIRECTIONS: [Delta<i32>; 4] = [Delta::UP, Delta::DOWN, Delta::LEFT, Delta::RIGHT];

fn do_rounds(points: &mut HashSet<PointS>, max_rounds: Option<usize>) -> usize {
    let mut initial_direction_index = 0;
    loop {
        if let Some(max_rounds) = max_rounds {
            if initial_direction_index >= max_rounds {
                return initial_direction_index;
            }
        }
        // println!("Iteration={}", initial_direction_index);
        let mut destinations_to_sources = HashMap::new();
        for point in points.iter() {
            // println!("point={:?}", point);
            if Delta::NEIGHBORS8
                .iter()
                .all(|direction| !points.contains(&(point + direction)))
            {
                // println!("No neighbors!");
                continue;
            }
            for direction in DIRECTIONS
                .iter()
                .cycle()
                .skip(initial_direction_index % 4)
                .take(4)
            {
                let perpendicular = direction.rotate_about_origin_deg(Rotation::Deg90);
                let is_clear = [-1, 0, 1].into_iter().all(|multiplier| {
                    let test_point = point + direction + (perpendicular * multiplier);
                    !points.contains(&test_point)
                });
                if is_clear {
                    // println!(
                    //     "Proposing to move from {:?} to {:?}",
                    //     point,
                    //     point + direction
                    // );
                    destinations_to_sources
                        .entry(point + direction)
                        .or_insert_with(Vec::new)
                        .push(*point);
                    break;
                    // } else {
                    //     println!("Cannot move from {:?} to {:?}", point, point + direction);
                }
            }
        }

        if destinations_to_sources.is_empty() {
            return initial_direction_index;
        }
        for (destination, sources) in destinations_to_sources.into_iter() {
            if sources.len() == 1 {
                points.remove(&sources[0]);
                points.insert(destination);
            }
        }
        initial_direction_index += 1;
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let initial_grid = Grid::from_str(input, "\n", None, |s| s == "#");
    let points = initial_grid
        .iter_with_points()
        .filter(|(_, has_elf)| **has_elf)
        .map(|(point, _)| point.cast().unwrap())
        .collect::<HashSet<_>>();
    let mut pt1_points = points.clone();
    do_rounds(&mut pt1_points, Some(10));
    // dbg!(Grid::from_signed_points(points.iter(), None));

    let (a, b) = PointS::get_bounding_box(pt1_points.iter());
    let num_empty_spots = (b - a + Delta::DOWN_RIGHT).area() - pt1_points.len() as i32;

    println!("Part 1: {}", num_empty_spots);

    let mut pt2_points = points.clone();
    let num_rounds = do_rounds(&mut pt2_points, None);
    println!("Part 2: {}", num_rounds + 1);
}
