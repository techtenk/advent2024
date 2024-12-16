extern crate core;

use std::env::args;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    if args().any(|arg| arg.contains("--all")) {
        day1::run();
        day2::run();
        day3::run();
    }
    day4::run();
}
