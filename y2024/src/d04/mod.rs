use util::grid::{Grid, Neighbors};
use util::point2::DeltaU;

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let grid = Grid::from_str(&input, "\n", None, |c| c.chars().next().unwrap());

    let p1 = grid
        .iter_with_points()
        .filter(|(_, char)| **char == 'X')
        .flat_map(|(start_point, _)| {
            grid.neighbors_with_values(start_point, Neighbors::Eight)
                .filter(|(_, c)| **c == 'M')
                .map(move |(neighbor_point, _)| (neighbor_point, neighbor_point - start_point))
        })
        .filter(|(point, delta)| {
            grid.get_option(point.checked_add(delta)) == Some(&'A')
                && grid.get_option(point.checked_add(&(delta * 2))) == Some(&'S')
        })
        .count();

    println!("Part 1: {}", p1);

    let cross_deltas = [
        (DeltaU::new(1, 1), DeltaU::new(-1, -1)),
        (DeltaU::new(-1, 1), DeltaU::new(1, -1)),
    ];
    let valid_results = [(Some(&'M'), Some(&'S')), (Some(&'S'), Some(&'M'))];

    let p2 = grid
        .iter_with_points()
        .filter(|(point, c)| {
            **c == 'A'
                && cross_deltas.iter().all(|(a, b)| {
                    valid_results.contains(&(
                        grid.get_option(point.checked_add(a)),
                        grid.get_option(point.checked_add(b)),
                    ))
                })
        })
        .count();

    println!("Part 2: {}", p2);
}
