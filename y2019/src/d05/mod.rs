use crate::computer::Computer;

pub fn main() {
    // let input = include_str!("example_input.txt").trim();
    // let input = input.split("\n").next().unwrap().trim();
    // for input_val in [0, 7, 8, 9].iter() {
    //     let mut example_computer = Computer::from_packed(input);
    //     example_computer.send_as_input(*input_val);
    //     example_computer.run();
    //     println!("example {} -> {:?}", input, example_computer.outputs());
    // }

    let input = include_str!("actual_input.txt").trim();

    let mut computer = Computer::from_packed(input);
    computer.send_as_input(1);
    computer.run().assert_finished();
    println!("{:?}", computer.outputs());

    let mut computer = Computer::from_packed(input);
    computer.send_as_input(5);
    computer.run().assert_finished();
    println!("{:?}", computer.outputs());
}
