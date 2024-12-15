use std::cmp::PartialEq;
use std::io::BufRead;
use crate::day2::Direction::{NEGATIVE, POSITIVE};

#[derive(PartialEq)]
enum Direction {
    POSITIVE,
    NEGATIVE,
}

pub(crate) fn run() {

    let safe_counter = count_safe_reports(false);
    println!("Number of safe reports (no dampener): {}", safe_counter);
    let safe_counter = count_safe_reports(true);
    println!("Number of safe reports (with dampener): {}", safe_counter);
}

fn count_safe_reports(dampener: bool) -> i32 {
    let input = include_bytes!("input.txt");

    let mut safe_counter = 0;
    for result in input.lines() {
        let line = result.unwrap();
        let mut lines_to_check = Vec::new();
        if dampener {
            for (i, _) in line.split_whitespace().enumerate() {
                let mut new_line = line.split_whitespace().collect::<Vec<&str>>();
                new_line.remove(i);
                lines_to_check.push(new_line.join(" "));
            }
        } else {
            lines_to_check.push(line);
        }
        if lines_to_check.iter().any(|line| check_line(line.to_string())) {
            safe_counter += 1;
        }
    }
    safe_counter
}

fn check_line(line: String) -> bool {
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
    safe
}