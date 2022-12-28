#![feature(array_zip)]
#![feature(array_methods)]
#![feature(variant_count)]
#![cfg_attr(not(all), allow(dead_code))]
#![cfg_attr(not(all), allow(unused_imports))]

#[macro_use]
extern crate impl_ops;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate lalrpop_util;
extern crate core;

use structopt::StructOpt;

mod util;
#[cfg(feature = "y2019")]
mod y2019;
#[cfg(feature = "y2020")]
mod y2020;
#[cfg(feature = "y2021")]
mod y2021;
#[cfg(feature = "y2022")]
mod y2022;

#[derive(Debug, StructOpt)]
#[structopt(name = "Main")]
struct Opt {
    #[structopt(default_value = "0")]
    year: u32,
    #[structopt(default_value = "0")]
    day: u32,
}

struct Year {
    year: u32,
    days: Vec<fn()>,
}

fn main() {
    let years = vec![
        #[cfg(feature = "y2019")]
        Year {
            year: 2019,
            days: vec![
                y2019::d01::main,
                y2019::d02::main,
                y2019::d03::main,
                y2019::d04::main,
                y2019::d05::main,
                y2019::d06::main,
                y2019::d07::main,
                y2019::d08::main,
                y2019::d09::main,
                y2019::d10::main,
                y2019::d11::main,
                y2019::d12::main,
                y2019::d13::main,
                y2019::d14::main,
                y2019::d15::main,
                y2019::d16::main,
            ],
        },
        #[cfg(feature = "y2020")]
        Year {
            year: 2020,
            days: vec![
                y2020::d01::main,
                y2020::d02::main,
                y2020::d03::main,
                y2020::d04::main,
                y2020::d05::main,
                y2020::d06::main,
                y2020::d07::main,
                y2020::d08::main,
                y2020::d09::main,
                y2020::d10::main,
                y2020::d11::main,
                y2020::d12::main,
                y2020::d13::main,
                y2020::d14::main,
                y2020::d15::main,
                y2020::d16::main,
                y2020::d17::main,
                y2020::d18::main,
                y2020::d19::main,
                y2020::d20::main,
            ],
        },
        #[cfg(feature = "y2021")]
        Year {
            year: 2021,
            days: vec![
                y2021::d01::main,
                y2021::d02::main,
                y2021::d03::main,
                y2021::d04::main,
                y2021::d05::main,
                y2021::d06::main,
                y2021::d07::main,
                y2021::d08::main,
                y2021::d09::main,
                y2021::d10::main,
                y2021::d11::main,
                y2021::d12::main,
                y2021::d13::main,
                y2021::d14::main,
                y2021::d15::main,
                y2021::d16::main,
                y2021::d17::main,
                y2021::d18::main,
                y2021::d19::main,
                y2021::d20::main,
                y2021::d21::main,
                y2021::d22::main,
                y2021::d23::main,
                y2021::d24::main,
                y2021::d25::main,
            ],
        },
        #[cfg(feature = "y2022")]
        Year {
            year: 2022,
            days: vec![
                // Comment for break
                y2022::d01::main,
                y2022::d02::main,
                y2022::d03::main,
                y2022::d04::main,
                y2022::d05::main,
                y2022::d06::main,
                y2022::d07::main,
                y2022::d08::main,
                y2022::d09::main,
                y2022::d10::main,
                y2022::d11::main,
                y2022::d12::main,
                y2022::d13::main,
                y2022::d14::main,
                y2022::d15::main,
                y2022::d16::main,
                y2022::d17::main,
                y2022::d18::main,
                y2022::d19::main,
                y2022::d20::main,
            ],
        },
    ];

    let opt = Opt::from_args();
    let year = if opt.year != 0 {
        opt.year
    } else {
        years.last().unwrap().year
    };
    let year = years
        .iter()
        .find(|y| y.year == year)
        .unwrap_or_else(|| panic!("Unknown year {}", year));
    let day = if opt.day != 0 {
        opt.day as usize
    } else {
        year.days.len() - 1
    };
    let day = year
        .days
        .get(day)
        .unwrap_or_else(|| panic!("Unknown day {} for year {}", day, year.year));
    day();
}
