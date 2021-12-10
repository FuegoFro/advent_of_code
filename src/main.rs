#[macro_use]
extern crate impl_ops;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate lalrpop_util;

use structopt::StructOpt;

mod util;
mod y2019;
mod y2020;
mod y2021;

#[derive(Debug, StructOpt)]
#[structopt(name = "Main")]
struct Opt {
    #[structopt(default_value = "2021")]
    year: u32,
    #[structopt(default_value = "10")]
    day: u32,
}

fn main() {
    let opt = Opt::from_args();
    match opt.year {
        2019 => match opt.day {
            1 => y2019::d01::main(),
            2 => y2019::d02::main(),
            3 => y2019::d03::main(),
            4 => y2019::d04::main(),
            5 => y2019::d05::main(),
            6 => y2019::d06::main(),
            7 => y2019::d07::main(),
            8 => y2019::d08::main(),
            9 => y2019::d09::main(),
            10 => y2019::d10::main(),
            11 => y2019::d11::main(),
            12 => y2019::d12::main(),
            13 => y2019::d13::main(),
            14 => y2019::d14::main(),
            15 => y2019::d15::main(),
            16 => y2019::d16::main(),
            _ => println!("Unknown day {} for year {}", opt.day, opt.year),
        },
        2020 => match opt.day {
            1 => y2020::d01::main(),
            2 => y2020::d02::main(),
            3 => y2020::d03::main(),
            4 => y2020::d04::main(),
            5 => y2020::d05::main(),
            6 => y2020::d06::main(),
            7 => y2020::d07::main(),
            8 => y2020::d08::main(),
            9 => y2020::d09::main(),
            10 => y2020::d10::main(),
            11 => y2020::d11::main(),
            12 => y2020::d12::main(),
            13 => y2020::d13::main(),
            14 => y2020::d14::main(),
            15 => y2020::d15::main(),
            16 => y2020::d16::main(),
            17 => y2020::d17::main(),
            18 => y2020::d18::main(),
            19 => y2020::d19::main(),
            20 => y2020::d20::main(),
            _ => println!("Unknown day {} for year {}", opt.day, opt.year),
        },
        2021 => match opt.day {
            1 => y2021::d01::main(),
            2 => y2021::d02::main(),
            3 => y2021::d03::main(),
            4 => y2021::d04::main(),
            5 => y2021::d05::main(),
            6 => y2021::d06::main(),
            7 => y2021::d07::main(),
            8 => y2021::d08::main(),
            9 => y2021::d09::main(),
            10 => y2021::d10::main(),
            _ => println!("Unknown day {} for year {}", opt.day, opt.year),
        },
        _ => println!("Unknown year {}", opt.year),
    };
}
