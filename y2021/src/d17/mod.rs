use itertools::Itertools;
use recap::Recap;
use serde::Deserialize;

#[derive(Deserialize, Recap)]
// target area: x=20..30, y=-10..-5
#[recap(
    regex = r"target area: x=(?P<x_start>-?\d+)..(?P<x_end>-?\d+), y=(?P<y_start>-?\d+)..(?P<y_end>-?\d+)"
)]
struct TargetArea {
    x_start: i32,
    x_end: i32,
    y_start: i32,
    y_end: i32,
}

/// Iterate through a bunch of possible vertical velocities and find ones that work, as well a
/// which time steps that vertical velocity ended up being in the target area.
///
/// Doesn't handle positive target area y
fn valid_vertical_velocities(target_area: &TargetArea) -> Vec<(i32, Vec<u32>)> {
    let mut valid_vertical_velocities = vec![];
    let mut next_initial_velocity = target_area.y_start;
    // Kinda arbitrary, probably enough iterations. Need to handle skipped velocities
    for _ in 0..(10 * -target_area.y_start) {
        // TODO - Could probably use geometric progression formula???
        let mut height = 0;
        let mut velocity = next_initial_velocity;
        let mut steps_in_target = vec![];
        let mut step = 0;
        while height >= target_area.y_start {
            height += velocity;
            velocity -= 1;
            if target_area.y_start <= height && height <= target_area.y_end {
                steps_in_target.push(step);
            }
            step += 1;
        }
        if !steps_in_target.is_empty() {
            valid_vertical_velocities.push((next_initial_velocity, steps_in_target))
        }
        next_initial_velocity += 1;
    }
    valid_vertical_velocities
}

/// Run the vertical part of the simulation to determine the maximum height it reaches, given the
/// initial vertical velocity.
///
/// Yes this could have been calculated in valid_vertical_velocities, but it's doing enough already.
fn get_max_height_for_initial_velocity(initial_velocity: i32) -> i32 {
    let mut height = 0;
    let mut velocity = initial_velocity;
    while velocity > 0 {
        height += velocity;
        velocity -= 1;
    }
    height
}

/// Find which initial horizontal velocities will be in the target area at the given time step.
///
/// Doesn't handle negative target area x
fn valid_horizontal_velocities_for_step(target_step: u32, target_area: &TargetArea) -> Vec<i32> {
    let mut next_initial_velocity = 0;
    let mut valid_horizontal_velocities = vec![];
    // Kinda arbitrary, probably enough iterations.
    for _ in 0..=(target_area.x_end * 10) {
        let mut position = 0;
        let mut velocity = next_initial_velocity;
        for _ in 0..=target_step {
            position += velocity;
            if velocity > 0 {
                velocity -= 1;
            }
        }
        if target_area.x_start <= position && position <= target_area.x_end {
            valid_horizontal_velocities.push(next_initial_velocity);
        }
        next_initial_velocity += 1;
    }
    valid_horizontal_velocities
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace("\r", "");

    let target_area: TargetArea = input.parse().unwrap();

    let vertical_vs = valid_vertical_velocities(&target_area);

    let max_height = get_max_height_for_initial_velocity(vertical_vs.last().unwrap().0);
    println!("Part 1: {}", max_height);

    /*
    We have a bunch of (<initial vertical velocity>, <which time steps in target area>) pairs.
    Since the calculation of "what horizontal velocities are valid" only relies on what time step
    it's aiming to be in the target area, we can optimize by calculating possible horizontal
    velocities once per time step.

    So, we reformat the pairs to (<time step>, <initial vertical velocities that will be in the
    target at the time step>) pairs, then calculate the initial horizontal velocities for that
    time step, and finally get all combinations of those horizontal/vertical velocities.

    Since a given horizontal/vertical velocity could be in the target area for multiple time steps,
    (which is what we were operating on/grouping by), we need to make sure our velocity pairs are
    unique before we count them.
    */
    let all_velocities = vertical_vs
        .into_iter()
        .flat_map(|(vert_v, steps_in_target)| steps_in_target.into_iter().map(move |s| (s, vert_v)))
        .into_group_map()
        .into_iter()
        .flat_map(|(step, vert_vs)| {
            valid_horizontal_velocities_for_step(step, &target_area)
                .into_iter()
                .cartesian_product(vert_vs.into_iter())
        })
        .unique()
        // Made it easier to debug
        .sorted()
        .collect_vec();

    println!("Part 2: {}", all_velocities.len());
}
