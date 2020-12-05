#[derive(Debug, StructOpt)]
struct Opt {
    day: u32
}

fn main() {
    let opt = Opt::from_args();
    match opt.day {
        1 => day1(),
        x => println!("Unknown day {}", x),
    };
}
