use crate::computer::Computer;

pub fn main() {
    let input = include_str!("example_input.txt").trim();
    for line in input.split("\n") {
        let mut computer_example = Computer::from_packed(line);
        computer_example.run().assert_finished();
        println!("{:?}", computer_example.memory());
    }

    let input = include_str!("actual_input.txt").trim();

    let mut computer_pt1 = Computer::from_packed(input);
    computer_pt1.write_memory(1, 12);
    computer_pt1.write_memory(2, 02);
    computer_pt1.run().assert_finished();
    println!("{}", computer_pt1.get_value_at(0));

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut computer = Computer::from_packed(input);
            computer_pt1.write_memory(1, noun);
            computer_pt1.write_memory(2, verb);
            computer.run().assert_finished();
            if computer.get_value_at(0) == 19690720 {
                println!("{}", noun * 100 + verb);
                return;
            }
        }
    }
}
