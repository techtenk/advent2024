use std::cmp::PartialEq;
use std::io::BufRead;
use crate::day2::Direction::{NEGATIVE, POSITIVE};

#[derive(PartialEq)]
enum Direction {
    POSITIVE,
    NEGATIVE,
}

pub(crate) fn run() {
    let input = include_bytes!("input.txt");

    let mut safe_counter = 0;
    for result in input.lines() {
        let line = result.unwrap();
        let mut cursor = line.split_whitespace();
        let mut safe = true;
        let mut previous = None;
        let mut direction = None;
        while let Some(next_itr) = cursor.next() {
            let next = next_itr.parse::<i32>().unwrap();
            if previous == None {
                previous = Some(next);
                continue;
            }
            if !direction.is_some() {
                direction = match next - previous.unwrap() {
                    -1000..=0 => Some(NEGATIVE),
                    1..1000 => Some(POSITIVE),
                    _ => None,
                };
            }
            let diff = next - previous.unwrap();

            if diff.is_positive() && direction == Some(POSITIVE) && diff.abs() <= 3 {
                previous = Some(next);
                continue;
            } else if diff.is_negative() && direction == Some(NEGATIVE) && diff.abs() <= 3 {
                previous = Some(next);
                continue;
            }
            safe = false;
            break;
        }
        if let Ok(_) = std::env::var("AOC_DEBUG") {
            println!("Line {} is safe: {}", line, safe);
        }
        if safe {
            safe_counter += 1;
        }
    }
    println!("Number of safe reports: {}", safe_counter);
}