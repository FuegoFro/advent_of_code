#[macro_use]
extern crate impl_ops;
#[macro_use]
extern crate lazy_static;

use structopt::StructOpt;

mod y2019;
mod y2020;

#[derive(Debug, StructOpt)]
#[structopt(name = "Main")]
struct Opt {
    #[structopt(default_value = "2020")]
    year: u32,
    #[structopt(default_value = "7")]
    day: u32,
}

fn main() {
    let opt = Opt::from_args();
    match opt.year {
        2020 => match opt.day {
            1 => y2020::d1::main(),
            2 => y2020::d2::main(),
            3 => y2020::d3::main(),
            4 => y2020::d4::main(),
            5 => y2020::d5::main(),
            6 => y2020::d6::main(),
            7 => y2020::d7::main(),
            _ => println!("Unknown day {} for year {}", opt.day, opt.year),
        },
        2019 => match opt.day {
            1 => y2019::d1::main(),
            2 => y2019::d2::main(),
            _ => println!("Unknown day {} for year {}", opt.day, opt.year),
        },
        _ => println!("Unknown year {}", opt.year),
    };
}