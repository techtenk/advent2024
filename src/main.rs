extern crate core;

use std::env::{args};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
pub(crate) mod utils;
mod day9;
mod day10;
mod day11;
mod day12;

fn main() {
    let test_mode = args().any(|arg| arg == "--test");
    if args().any(|arg| arg.contains("--all")) {
        day1::run();
        day2::run();
        day3::run();
        day4::run();
        day5::run();
        day6::run(false);
        day7::run(false);
        day8::run(false);
        day9::run(false);
        day10::run(false);
        day11::run(false);
    }
    day12::run(test_mode);
}
