use itertools::Itertools;

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    // let width = 2;
    // let height = 2;
    let input = include_str!("actual_input.txt").trim();
    let width = 25;
    let height = 6;

    let layer_size = width * height;
    let num_layers = input.len() / layer_size;
    let all_nums = input.bytes().map(|c| c - b'0').collect_vec();
    let layers = (0..num_layers)
        .map(|layer_idx| &all_nums[layer_idx * layer_size..(layer_idx + 1) * layer_size])
        .collect_vec();

    pt1(&layers);
    pt2(&layers, width);
}

fn pt1(layers: &[&[u8]]) {
    let mut min_zeroes = usize::max_value();
    let mut min_layer_idx = usize::max_value();

    for (layer_idx, layer) in layers.iter().enumerate() {
        let num_zeroes = layer.iter().filter(|d| **d == 0).count();
        if num_zeroes < min_zeroes {
            min_zeroes = num_zeroes;
            min_layer_idx = layer_idx;
        }
    }
    let pt1_num_ones = layers[min_layer_idx].iter().filter(|c| **c == 1).count();
    let pt1_num_twos = layers[min_layer_idx].iter().filter(|c| **c == 2).count();
    println!("{}", pt1_num_ones * pt1_num_twos);
}

fn pt2(layers: &Vec<&[u8]>, layer_width: usize) {
    let mut image = vec![2; layers[0].len()];
    for layer in layers {
        for (a, b) in image.iter_mut().zip_eq(layer.iter()) {
            if *a == 2 {
                *a = *b;
            }
        }
    }
    for row in &image.iter().chunks(layer_width) {
        for char in row {
            if *char == 1 {
                print!("X");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
