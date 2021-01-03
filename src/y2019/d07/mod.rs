use crate::y2019::computer::{Computer, ComputerExitStatus, Word};
use itertools::Itertools;
use std::cmp::max;

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    let input = include_str!("actual_input.txt").trim();

    pt1(input);
    pt2(input);
}

fn pt1(input: &str) {
    let mut max_value = Word::min_value();
    for phases in vec![0, 1, 2, 3, 4].into_iter().permutations(5) {
        let mut value = 0;
        for phase in phases {
            let mut computer = Computer::from_packed(input);
            computer.send_as_input(phase);
            computer.send_as_input(value);
            computer.run().assert_finished();
            assert_eq!(computer.outputs().len(), 1);
            value = computer.outputs()[0];
        }
        max_value = max(max_value, value);
    }
    println!("{}", max_value);
}

fn pt2(input: &str) {
    let mut max_value = Word::min_value();
    for phases in vec![5, 6, 7, 8, 9].into_iter().permutations(5) {
        let mut computers = phases
            .iter()
            .map(|phase| {
                let mut computer = Computer::from_packed(input);
                computer.send_as_input(*phase);
                computer
            })
            .collect_vec();

        let mut values = vec![0];
        let mut most_recent_status: Option<ComputerExitStatus> = None;
        while most_recent_status != Some(ComputerExitStatus::Finished) {
            most_recent_status = None;
            for computer in computers.iter_mut() {
                for value in values {
                    computer.send_as_input(value);
                }
                let status = Some(computer.run());
                if most_recent_status.is_none() {
                    most_recent_status = status;
                } else {
                    assert_eq!(most_recent_status, status);
                }
                values = computer.drain_outputs();
            }
        }
        assert_eq!(values.len(), 1);
        let value = values[0];
        max_value = max(max_value, value);
    }
    println!("{}", max_value);
}
