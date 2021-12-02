use crate::util::{p_u64, split_once};
use binary_search::{binary_search, Direction};
use num::integer::div_ceil;
use std::cmp::min;
use std::collections::HashMap;
use std::iter;

struct Recipe<'a> {
    output_count: u64,
    inputs: Vec<(u64, &'a str)>,
}

fn parse_ingredient(raw: &str) -> (u64, &str) {
    let (count, name) = split_once(raw, " ");
    (p_u64(count), name)
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let recipes = input
        .lines()
        .map(|l| {
            let (inputs, output) = split_once(l, " => ");
            let (output_count, output_name) = parse_ingredient(output);
            (
                output_name,
                Recipe {
                    output_count,
                    inputs: inputs.split(", ").map(parse_ingredient).collect(),
                },
            )
        })
        .chain(iter::once((
            "ORE",
            Recipe {
                output_count: 1,
                inputs: Vec::new(),
            },
        )))
        .collect::<HashMap<_, _>>();

    let ore_needed_for_one = get_ore_needed_for_fuel(&recipes, 1);
    println!("{}", ore_needed_for_one);

    // Exponential growth to find lower/upper bound
    let (lower_fuel, upper_fuel) = get_lower_upper_bounds(&recipes);
    // Binary search to find actual
    let ((low, _), (high, _)) = binary_search((lower_fuel, ()), (upper_fuel, ()), |fuel| {
        if get_ore_needed_for_fuel(&recipes, fuel) < 1000000000000 {
            Direction::Low(())
        } else {
            Direction::High(())
        }
    });
    println!("{} -> {}", low, get_ore_needed_for_fuel(&recipes, low));
    println!("{} -> {}", high, get_ore_needed_for_fuel(&recipes, high));
}

fn get_lower_upper_bounds(recipes: &HashMap<&str, Recipe>) -> (u64, u64) {
    let mut prev_fuel = 1;
    loop {
        let current_fuel = prev_fuel * 2;
        let current_ore = get_ore_needed_for_fuel(&recipes, current_fuel);
        if current_ore >= 1000000000000 {
            return (prev_fuel, current_fuel);
        }
        prev_fuel = current_fuel;
    }
}

fn get_ore_needed_for_fuel(recipes: &HashMap<&str, Recipe>, fuel_count: u64) -> u64 {
    let mut needed = vec![(fuel_count, "FUEL")];
    let mut produced: HashMap<&str, u64> = HashMap::new();
    let mut extras: HashMap<&str, u64> = HashMap::new();
    while let Some((mut next_count, next_name)) = needed.pop() {
        if let Some(extra_count) = extras.get_mut(next_name) {
            let amount_from_extras = min(*extra_count, next_count);
            *extra_count -= amount_from_extras;
            next_count -= amount_from_extras;
        }
        let recipe = &recipes[next_name];
        let num_times_to_run = div_ceil(next_count, recipe.output_count);
        let produced_count = num_times_to_run * recipe.output_count;
        *produced.entry(next_name).or_default() += produced_count;
        let extra_outputs = produced_count - next_count;
        *extras.entry(next_name).or_default() += extra_outputs;
        for (input_count, input_name) in recipe.inputs.iter() {
            needed.push((input_count * num_times_to_run, input_name));
        }
    }
    produced["ORE"]
}
