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
    println!("Found {:?} antinodes", antinodes);
    antinodes.dedup();
    println!("Found {} antinodes", antinodes.len());
}
fn find_antinodes(node1: (usize, usize), node2: (usize, usize)) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
    let (x1, y1) = node1;
    let (x2, y2) = node2;
    let row_diff = x2 as isize - x1 as isize;
    let col_diff = y2 as isize - y1 as isize;
    // add row diff to the larger row and col diff to the larger col to make antinode 1
    let (x3, x4) =
        'x:{
            if row_diff > 0 {
                // x2 is the larger row
                let x3 = x2 + row_diff as usize;
                if x1 as isize >= row_diff {
                    let x4 = x1 - row_diff as usize;
                    break 'x (Some(x3), Some(x4));
                }
                break 'x (Some(x3), None)
            }
            // x1 is the larger row
            let x3 = x1 + row_diff as usize;
            if x2 as isize >= row_diff {
                let x4 = x2 - row_diff as usize;
                break 'x (Some(x3), Some(x4));
            }
            (Some(x3), None)
        };

    let (y3, y4) =
        'y: {
            if col_diff > 0 {
                // y2 is the larger col
                let y3 = y2 + col_diff as usize;
                if y1 as isize >= col_diff {
                    let y4 = y1 - col_diff as usize;
                    break 'y (Some(y3), Some(y4));
                }
                break 'y (Some(y3), None)
            }
            // y1 is the larger col
            let y3 = y1 + col_diff.abs() as usize;
            if y2 as isize >= col_diff.abs() {
                let y4 = y2 - col_diff.abs() as usize;
                break 'y (Some(y3), Some(y4));
            }
            (Some(y3), None)
        };
    match (x3, x4, y3, y4) {
        (Some(x3), Some(x4), Some(y3), Some(y4)) => (Some((x3, y3)), Some((x4, y4))),
        (Some(x3), Some(y3), _, _) => (Some((x3, y3)), None),
        (_, _, Some(x4), Some(y4)) => (Some((x4, y4)), None),
        _ => (None, None),
    }
}