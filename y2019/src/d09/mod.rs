use crate::computer::Computer;

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    // let mut example_computer = Computer::from_packed(input);
    // example_computer.run().assert_finished();
    // println!("{:?}", example_computer.outputs());

    let input = include_str!("actual_input.txt").trim();
    let mut computer = Computer::from_packed(input);
    computer.send_as_input(1);
    computer.run().assert_finished();
    println!("{:?}", computer.outputs());

    let mut computer = Computer::from_packed(input);
    computer.send_as_input(2);
    computer.run().assert_finished();
    println!("{:?}", computer.outputs());
}
