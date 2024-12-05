extern crate core;

use std::env::args;

mod day1;
mod day2;

fn main() {
    if args().any(|arg| arg.contains("--all")) {
        day1::run();
    }
    day2::run();
}
