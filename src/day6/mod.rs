use std::collections::HashSet;

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
    'bounds: loop {
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

    println!("Number of spaces visited: {}", visited.len());

}
