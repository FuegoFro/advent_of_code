use util::grid::Grid;
use util::p_u32;
use util::point2::Delta;

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace("\r", "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let grid = Grid::from_str(input, "\n", None, p_u32);

    let visible_trees: u32 = grid
        .iter_with_points()
        .map(|(point, val)| {
            let all_blocking = Delta::NEIGHBORS4.iter().all(|delta| {
                let mut total_delta = *delta;
                while let Some(p) = point.checked_add(&total_delta) {
                    if p.x >= grid.width() || p.y >= grid.height() {
                        break;
                    }
                    if grid[p] >= *val {
                        return true;
                    }
                    total_delta += delta;
                }
                false
            });
            u32::from(!all_blocking)
        })
        .sum();

    println!("Part 1: {}", visible_trees);

    let best_viewing_score: u32 = grid
        .iter_with_points()
        .map(|(point, val)| {
            Delta::NEIGHBORS4
                .iter()
                .map(|delta| {
                    let mut total_delta = *delta;
                    let mut score = 0;
                    while let Some(p) = point.checked_add(&total_delta) {
                        if p.x >= grid.width() || p.y >= grid.height() {
                            break;
                        }
                        score += 1;
                        if grid[p] >= *val {
                            break;
                        }
                        total_delta += delta;
                    }
                    score
                })
                .product()
        })
        .max()
        .unwrap();

    println!("Part 2: {}", best_viewing_score);
}
