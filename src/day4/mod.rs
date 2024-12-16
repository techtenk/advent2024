use std::io::BufRead;

pub(crate) fn run() {
    let input = include_bytes!("input.txt");
    let width = input.lines().next().unwrap().unwrap().len();
    let mut puzzle = Vec::new();
    for line in input.lines() {
        let line = line.unwrap();

        for c in line.chars() {
            puzzle.push(c);
        }
    }
    let mut count: u16 = 0;
    // check for forward and backward sequences
    for i in 0..puzzle.len() {
        if i % width <= width - 4 {
            count += check_for_xmas([i, i+1, i+2, i+3], &puzzle);
        }
    }

    // check for vertical sequences
    for i in 0..puzzle.len() {
        if i < puzzle.len() - (3 * width) {
            count += check_for_xmas([i, i + width, i + width * 2, i + width * 3], &puzzle);
        }
    }

    // check for diagonal sequences
    for i in 0..puzzle.len() {
        if i % width <= width - 4 && i < puzzle.len() - (3 * width) {
            count += check_for_xmas([i, i + width + 1, i + width * 2 + 2, i + width * 3 + 3], &puzzle);
        }
    }

    // check for reverse diagonal sequences
    for i in 0..puzzle.len() {
        if i % width >= 3 && i < puzzle.len() - (3 * width) {
            count += check_for_xmas([i, i + width - 1, i + width * 2 - 2, i + width * 3 - 3], &puzzle);
        }
    }
    println!("Number of XMAS and SAMX sequences: {}", count);

    let mut part2count = 0;
    // part 2
    for index in 0..puzzle.len() - 2*width {
        if index % width <= width - 3 {
            let grid = [puzzle[index], puzzle[index + 1], puzzle[index + 2], puzzle[index + width], puzzle[index + width + 1], puzzle[index + width + 2], puzzle[index + width * 2], puzzle[index + width * 2 + 1], puzzle[index + width * 2 + 2]];
            if check_for_x(grid) {
                // println!("Found XMAS and SAMX grid at index: {}", index);
                part2count += 1;
            }
        }
    }

    println!("Number of XMAS and SAMX grids: {}", part2count);
}

fn check_for_xmas(indexes: [usize; 4], puzzle: &Vec<char>) -> u16 {
    let mut xmas = 0;
    let word = format!("{}{}{}{}", puzzle[indexes[0]], puzzle[indexes[1]], puzzle[indexes[2]], puzzle[indexes[3]]);
    // println!("Checking word: {}", word);
    if word.eq("XMAS") {
        xmas += 1;
        // println!("Found XMAS at indexes: {:?}", indexes);
    }
    if word.eq("SAMX") {
        xmas += 1;
        // println!("Found SAMX at indexes: {:?}", indexes);
    }
    xmas
}

fn check_for_x(grid: [char; 9]) -> bool {
    let diag1 = match format!("{}{}{}", grid[0], grid[4], grid[8]).as_str() {
        "MAS" => true,
        "SAM" => true,
        _ => false,
    };
    let diag2 = match format!("{}{}{}", grid[2], grid[4], grid[6]).as_str() {
        "MAS" => true,
        "SAM" => true,
        _ => false,
    };
    diag1 && diag2
}