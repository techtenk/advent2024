use std::collections::HashSet;
use crate::utils::grid::Grid;

pub(crate) fn run(test: bool) {
    let real_input = include_str!("input.txt");
    let test_input = include_str!("test.txt");
    let input = if test { test_input } else { real_input };

    let map = Grid::from(input);

    // get the trailheads
    let mut trailheads = Vec::new();
    for row in 0..map.get_height() {
        for col in 0..map.get_width() {
            if map.get(row, col) == Some('0') {
                trailheads.push((row, col));
            }
        }
    }

    // iterate through trailheads and then recursively check all it's adjacent cells to see if it is height + 1
    let mut trail_score = 0;
    let mut trail_count = 0;
    for trailhead in trailheads {
        let (num, summits) = check_adjacent(&map, trailhead);
        // println!("Trailhead: {:?} has {} trails", trailhead, num);
        trail_count  += summits.len();
        trail_score += num;
    }
    // trail_score = check_adjacent(&map, trailheads[0]);
    println!("The number of trails that reach the summit is: {}", trail_count);
    println!("The total trail score of trails that reach the summit is: {}", trail_score);
}

fn check_adjacent(map: &Grid<char>, cell: (usize, usize)) -> (usize, HashSet<(usize, usize)>) {
    // println!("Checking cell: {:?} with value: {}", cell, map.get(cell.0, cell.1).unwrap());
    let (row, col) = cell;
    // base case, we are at the top, return 1
    if map.get(row, col) == Some('9') {
        let mut set = HashSet::new();
        set.insert(cell);
        return (1, set);
    }

    // for each adjacent cell that == cell_value + 1, recurse
    let cell_value = map.get(row, col).unwrap().to_digit(10).unwrap();
    let (left, right, up, down) = map.get_adjacent(row, col);
    // println!("Found adjacents: {:?}, {:?}, {:?}, {:?}", left, right, up, down);
    let mut trail_score = 0;
    let mut summits = HashSet::new();
    let mut check_direction = |c: char, next_step: (usize, usize) | {
        if c.to_digit(10).unwrap() == cell_value + 1 {
            let (num, summit) = check_adjacent(map, next_step);
            for s in summit.iter() {
                // println!("Is {:?} in summits? {}", s, summits.contains(s));
                if !summits.contains(s) {

                    summits.insert(*s);
                }
            }
            trail_score += num;
        }

    };
    match left {
        Some(c) => {
            check_direction(c, (row, col - 1));
        },
        None => {}
    }
    match right {
        Some(c) => {
            check_direction(c, (row, col + 1));
        },
        None => {}
    }
    match up {
        Some(c) => {
            check_direction(c, (row - 1, col));
        },
        None => {}
    }
    match down {
        Some(c) => {
            check_direction(c, (row + 1, col));
        },
        None => {}
    }

    (trail_score, summits)
}