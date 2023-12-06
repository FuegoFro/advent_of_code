use itertools::Itertools;
use util::p_u64;

fn num_winning_options((time, distance): (u64, u64)) -> usize {
    // If T = race time
    //    D = distance to beat
    //    H = hold duration
    // Then we want any H such that (T-H)*H > D
    // Plugging that into Wolfram Alpha to solve for H gives us
    // https://www.wolframalpha.com/input?i=solve+%28T-H%29*H+%3E+D+for+H
    // 1/2 (T - sqrt(T^2 - 4 D))<H<1/2 (sqrt(T^2 - 4 D) + T)
    (0..time)
        .filter(|hold_duration| (time - hold_duration) * hold_duration > distance)
        .count()
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let (times_raw, distances_raw): (&str, &str) = input.lines().collect_tuple().unwrap();
    let times = times_raw.split_whitespace().skip(1).map(p_u64);
    let distances = distances_raw.split_whitespace().skip(1).map(p_u64);
    let races = times.zip(distances).collect_vec();

    // If T = race time
    //    D = distance to beat
    //    H = hold duration
    // Then we want any H such that (T-H)*H > D
    // Plugging that into Wolfram Alpha to solve for H gives us
    // https://www.wolframalpha.com/input?i=solve+%28T-H%29*H+%3E+D+for+H
    // 1/2 (T - sqrt(T^2 - 4 D))<H<1/2 (sqrt(T^2 - 4 D) + T)
    // ....... or just do it dumb because rust is fast

    let p1 = races
        .into_iter()
        .map(num_winning_options)
        .product::<usize>();

    println!("Part 1: {}", p1);

    let concat_time = times_raw
        .replace(' ', "")
        .split_once(':')
        .unwrap()
        .1
        .parse::<u64>()
        .unwrap();
    let concat_dist = distances_raw
        .replace(' ', "")
        .split_once(':')
        .unwrap()
        .1
        .parse::<u64>()
        .unwrap();

    let p2 = num_winning_options((concat_time, concat_dist));

    println!("Part 2: {}", p2);
}
