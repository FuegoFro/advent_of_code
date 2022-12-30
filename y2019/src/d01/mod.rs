use std::cmp::max;

fn calc_fuel_single(mass: i32) -> i32 {
    max(mass / 3 - 2, 0)
}

fn calc_fuel_total(mass: i32) -> i32 {
    let mut remaining_mass = mass;
    let mut total_fuel = 0;
    while remaining_mass > 0 {
        let single_fuel = calc_fuel_single(remaining_mass);
        total_fuel += single_fuel;
        remaining_mass = single_fuel;
    }

    total_fuel
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let total_fuel: i32 = input
        .split("\n")
        .map(|l| l.parse::<i32>().unwrap())
        .map(calc_fuel_total)
        .sum();

    println!("{}", total_fuel)
}
