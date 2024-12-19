use std::collections::HashMap;
use crate::utils::grid::Grid;
pub(crate) fn run(test: bool) {
    let real_input = include_str!("input.txt");
    let test_input = include_str!("test.txt");
    let input = if test { test_input } else { real_input };

    let width = input.lines().next().unwrap().len();
    let mut puzzle = Grid::new(width, width);
    for (line_no, line) in input.lines().enumerate() {
        for (col_no, c) in line.chars().enumerate() {
            puzzle.set(col_no, line_no, c);
        }
    }

    // index all the characters and their positions
    let mut map = HashMap::new();
    for row in 0..width {
        for col in 0..width {
            let c = puzzle.get(row, col);
            if c.is_some() {
                let e: &mut Vec<(usize, usize)> = map.entry(c.unwrap()).or_insert(Vec::new());
                e.push((row, col));
            }
        }
    }

    let mut antinodes = Vec::new();
    let mut line_antinodes = Vec::new();
    for c in map.keys().into_iter().copied() {
        if c == '.' {
            continue;
        }
        // println!("Nodes for {} are {:?}", c, map.get(&c).unwrap());
        // for each distict pair of nodes, calculate the two antinodes positions
        let nodes = map.get(&c).unwrap();
        for i in 0..nodes.len() {
            for j in i + 1..nodes.len() {
                let (x1, y1) = nodes[i];
                let (x2, y2) = nodes[j];
                let (antinode1, antinode2) = find_antinodes((x1, y1), (x2, y2));
                line_antinodes.append(&mut find_all_nodes_in_line((x1, y1), (x2, y2), width));
                if antinode1.is_some() {
                    let (x, y) = antinode1.unwrap();
                    if puzzle.is_in_bounds(x, y) {
                        antinodes.push(antinode1.unwrap());
                    }
                }
                if antinode2.is_some() {
                    let (x, y) = antinode2.unwrap();
                    if puzzle.is_in_bounds(x, y) {
                        antinodes.push(antinode2.unwrap());
                    }
                }
            }
        }
    }


    antinodes.sort();
    // println!("Found {:?} antinodes", antinodes);
    antinodes.dedup();
    println!("Found {} antinodes", antinodes.len());
    line_antinodes.sort();
    // println!("Found {:?} line antinodes", line_antinodes);
    line_antinodes.dedup();
    println!("Found {:?} line antinodes", line_antinodes.len());
}
fn find_antinodes(node1: (usize, usize), node2: (usize, usize)) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
    let (x1, y1) = node1;
    let (x2, y2) = node2;
    let dx = x2 as i32 - x1 as i32;
    let dy = y2 as i32 - y1 as i32;
    let antinode1 = (x1 as i32 - dx, y1 as i32 - dy);
    let antinode2 = (x2 as i32 + dx, y2 as i32 + dy);
    match (antinode1.0 >= 0, antinode1.1 >= 0, antinode2.0>= 0, antinode2.1 >= 0) {
        (true, true, true, true) => (Some((antinode1.0 as usize, antinode1.1 as usize)), Some((antinode2.0 as usize, antinode2.1 as usize))),
        (true, true, _, _) => (Some((antinode1.0 as usize, antinode1.1 as usize)), None),
        (_, _, true, true) => (None, Some((antinode2.0 as usize, antinode2.1 as usize))),
        _ => (None, None),
    }
}

fn find_all_nodes_in_line(node1: (usize, usize), node2: (usize, usize), width: usize) -> Vec<(usize, usize)> {
    let (x1, y1) = node1;
    let (x2, y2) = node2;
    let dx = x2 as i32 - x1 as i32;
    let dy = y2 as i32 - y1 as i32;
    let mut nodes = Vec::new();
    let mut x = x1 as i32;
    let mut y = y1 as i32;
    while x >= 0 && y >= 0 && x < width as i32 && y < width as i32 {
        nodes.push((x as usize, y as usize));
        x -= dx;
        y -= dy;
    }
    x = x2 as i32;
    y = y2 as i32;
    while x >= 0 && y >= 0 && x < width as i32 && y < width as i32 {
        nodes.push((x as usize, y as usize));
        x += dx;
        y += dy;
    }
    nodes
}