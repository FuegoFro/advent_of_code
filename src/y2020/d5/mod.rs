fn seat_to_id(seat: &str) -> u16 {
    let bin_str = seat
        .replace("F", "0")
        .replace("B", "1")
        .replace("L", "0")
        .replace("R", "1");
    u16::from_str_radix(&bin_str, 2).unwrap()
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    let mut seats = input.split("\n").map(|l| seat_to_id(l)).collect::<Vec<_>>();
    seats.sort();

    let max = seats.iter().max().unwrap();
    println!("{}", max);

    let my_seat = seats
        .iter()
        .fold((None, 0u16), |(res, prev), current| {
            (
                // Use the result we already have, otherwise see if this is a valid result
                res.or_else(|| {
                    if prev + 2 == *current {
                        Some(prev + 1)
                    } else {
                        None
                    }
                }),
                *current,
            )
        })
        .0
        .unwrap();
    println!("{}", my_seat);
}
