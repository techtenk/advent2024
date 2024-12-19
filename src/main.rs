extern crate core;

use std::env::{args, Args};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
pub(crate) mod utils;

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
    }
    day8::run(test_mode);
}
