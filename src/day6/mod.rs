use std::collections::HashSet;
use Box;
use std::error::Error;

enum DIRECTION {
    UP,
    DOWN,
    LEFT,
    RIGHT
}
pub(crate) fn run(test: bool) {
    let real_input = include_str!("input.txt");
    let test_input = include_str!("test.txt");
    let input = if test { test_input } else { real_input };
    let width = input.lines().next().unwrap().len() + 2; // add 1 to the width to account for the newline and carriage return

    let part1 = simulate(input).expect("Could not solve part 1!");
    println!("Number of spaces visited: {}", part1.len());

    // part 2
    // this is not very interesting to me, so I guess we can just brute force it
    // guess and check every spot that is in the visited set
    let mut obstruction_positions: Vec<(usize, usize)> = Vec::new();
    for (row, column) in part1 {
        // println!("Checking position: {}, {}", row, column);
        let mut new_input = input.clone().to_string();
        if new_input.chars().nth(row * width + column).unwrap() == '^' {
            // we are already at the starting position, no need to check this one
            continue;
        }
        new_input.replace_range((row * width + column)..(row * width + column + 1), "#");

        match simulate(&new_input) {
            Ok(visited) => {
                // found a way out, continue to the next position
                continue;
            },
            Err(_) => {
                // println!("Found an infinite loop at position: {}, {}", row, column);
                obstruction_positions.push((row, column));
            },
        }
    }
    println!("Number of obstruction positions: {}", obstruction_positions.len());
}

fn simulate(input: &str) -> Result<HashSet<(usize, usize)> , Box<dyn Error>> {
    let width = input.lines().next().unwrap().len() + 2; // add 1 to the width to account for the newline and carriage return

    let start_position = input.find(&['^','<','>','v']).unwrap();
    let mut current_row = start_position / width;
    let mut current_column = start_position % width;
    let mut current_direction = match input.chars().nth(start_position).unwrap() {
        '^' => DIRECTION::UP,
        'v' => DIRECTION::DOWN,
        '<' => DIRECTION::LEFT,
        '>' => DIRECTION::RIGHT,
        _ => panic!("Invalid character found at starting position"),
    };
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut steps = 0;
    'bounds: loop {
        steps += 1;
        if steps > 20000 {
            return Err("Infinite loop detected!".into());
        }
        // mark this position as visited
        visited.insert((current_row, current_column));
        // check if we need to turn (or if we are done) before continuing
        let next_pos = match current_direction {
            DIRECTION::UP => {
                if current_row == 0 { break 'bounds; }
                (current_row - 1, current_column)
            },
            DIRECTION::DOWN => (current_row + 1, current_column),
            DIRECTION::LEFT => {
                if current_column == 0 { break 'bounds; }
                (current_row, current_column - 1)
            },
            DIRECTION::RIGHT => (current_row, current_column + 1),
        };

        let next_char = input.chars().nth(next_pos.0 * width + next_pos.1);
        if next_char.is_none() {
            break 'bounds;
        }
        match next_char.unwrap() {
            '.'|'<'|'>'|'v'|'^' => {
                // keep going in the same direction
                match current_direction {
                    DIRECTION::UP => {
                        current_row -= 1;
                    },
                    DIRECTION::DOWN => {
                        current_row += 1;
                    },
                    DIRECTION::LEFT => {
                        current_column -= 1;
                    },
                    DIRECTION::RIGHT => {
                        current_column += 1;
                    },
                }
            },
            '#' => {
                // turn right
                current_direction = match current_direction {
                    DIRECTION::UP => DIRECTION::RIGHT,
                    DIRECTION::DOWN => DIRECTION::LEFT,
                    DIRECTION::LEFT => DIRECTION::UP,
                    DIRECTION::RIGHT => DIRECTION::DOWN,
                };
            },
            _ => {
                // we are done
                break 'bounds;
            },
        }
    }

    Ok(visited)
}
