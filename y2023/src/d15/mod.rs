use itertools::Itertools;
use recap::Recap;
use serde::Deserialize;

use util::p_u32;

#[derive(Recap, Deserialize)]
#[recap(regex = r"(?P<label>\w+)(?P<op>[=-])(?P<arg>\d*)")]
struct Instruction {
    label: String,
    op: String,
    arg: String,
}

#[derive(Default)]
struct Box {
    lenses: Vec<(String, u32)>,
    // focuses: Vec<u32>,
    // names_to_indices: HashMap<String, usize>,
}

impl Box {
    fn label_position(&self, target_label: &String) -> Option<usize> {
        self.lenses
            .iter()
            .position(|(label, _)| label == target_label)
    }

    fn calc_scores(&self, box_idx: usize) -> usize {
        /*
        One plus the box number of the lens in question.
        The slot number of the lens within the box: 1 for the first lens, 2 for the second lens, and so on.
        The focal length of the lens.
        */
        self.lenses
            .iter()
            .enumerate()
            .map(|(lens_idx, (_, focus))| (box_idx + 1) * (lens_idx + 1) * (*focus as usize))
            .sum::<usize>()
    }
}

pub fn main() {
    // let input = include_str!("example_input.txt").trim().replace('\r', "");
    let input = include_str!("actual_input.txt").trim().replace('\r', "");

    let p1 = input.split(',').map(do_hash).sum::<usize>();

    println!("Part 1: {}", p1);

    let mut boxes = (0..256).map(|_| Box::default()).collect_vec();
    for instruction in input.split(',').map(|i| i.parse::<Instruction>().unwrap()) {
        let b = boxes.get_mut(do_hash(&instruction.label)).unwrap();
        match instruction.op.as_str() {
            "-" => {
                if let Some(idx) = b.label_position(&instruction.label) {
                    b.lenses.remove(idx);
                }
            }
            "=" => {
                let arg = p_u32(instruction.arg.as_str());
                if let Some(idx) = b.label_position(&instruction.label) {
                    b.lenses[idx].1 = arg;
                } else {
                    b.lenses.push((instruction.label, arg))
                }
            }
            _ => panic!("Unknown op {}", instruction.op),
        }
    }

    let p2 = boxes
        .iter()
        .enumerate()
        .map(|(i, b)| b.calc_scores(i))
        .sum::<usize>();

    println!("Part 2: {}", p2);
}

fn do_hash(s: &str) -> usize {
    /*
    Determine the ASCII code for the current character of the string.
    Increase the current value by the ASCII code you just determined.
    Set the current value to itself multiplied by 17.
    Set the current value to the remainder of dividing itself by 256.
    */
    s.bytes().fold(0, |h, b| ((h + b as usize) * 17) % 256)
}
