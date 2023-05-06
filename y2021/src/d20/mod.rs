use itertools::Itertools;
use std::collections::HashSet;
use util::grid::Grid;
use util::point2::{DeltaS, PointS, PointU};

fn enhance(
    tracked_pixels: HashSet<PointS>,
    tracking_light: bool,
    replacements: &[bool],
) -> (HashSet<PointS>, bool) {
    let mut enhanced = HashSet::new();
    let enhanced_tracking_light = if replacements[0] {
        assert!(!replacements[511]);
        !tracking_light
    } else {
        true
    };
    let (min, max) = PointS::get_bounding_box(tracked_pixels.iter());
    for y in min.y - 1..=max.y + 1 {
        for x in min.x - 1..=max.x + 1 {
            let p = PointS::new(x, y);
            let replacement_idx = DeltaS::NEIGHBORS9
                .iter()
                .map(|d| tracked_pixels.contains(&(p + d)) == tracking_light)
                .map(usize::from)
                .fold(0, |acc, v| (acc << 1) + v);
            if replacements[replacement_idx] == enhanced_tracking_light {
                enhanced.insert(p);
            }
        }
    }
    (enhanced, enhanced_tracking_light)
}

#[allow(dead_code)]
fn print_image(image: &HashSet<PointS>) {
    if image.is_empty() {
        println!("EMPTY IMAGE");
        return;
    }
    let (min, max) = PointS::get_bounding_box(image.iter());
    let mut grid = Grid::empty((max.x - min.x + 1) as usize, (max.y - min.y + 1) as usize);
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let p = PointS::new(x, y);
            grid[PointU::ORIGIN + (p - min).cast().unwrap()] =
                if image.contains(&p) { '#' } else { '.' };
        }
    }
    dbg!(grid);
}

fn enhance_n(light_pixels: HashSet<PointS>, n: usize, replacements: &[bool]) -> HashSet<PointS> {
    assert_eq!(n % 2, 0, "Must enhance an even number of times");
    let mut tracked_pixels = light_pixels;
    let mut tracking_light = true;
    for _ in 0..n {
        let (enhanced_tracked_pixels, enhanced_tracking_light) =
            enhance(tracked_pixels, tracking_light, replacements);
        tracked_pixels = enhanced_tracked_pixels;
        tracking_light = enhanced_tracking_light;
    }
    tracked_pixels
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let (replacements_raw, grid_raw) = input.split_once("\n\n").unwrap();
    let replacements = replacements_raw.chars().map(|c| c == '#').collect_vec();
    let initial_grid = Grid::from_str(grid_raw, "\n", None, |v| v == "#");
    let light_pixels = initial_grid
        .iter_with_points()
        // Keep the lit points only
        .filter(|(_, v)| **v)
        // Convert from unsigned to signed
        .map(|(p, _)| PointS::ORIGIN + (p - PointU::ORIGIN).cast().unwrap())
        .collect::<HashSet<_>>();

    println!(
        "Part 1: {}",
        enhance_n(light_pixels.clone(), 2, &replacements).len()
    );

    println!(
        "Part 2: {}",
        enhance_n(light_pixels, 50, &replacements).len()
    );
}
