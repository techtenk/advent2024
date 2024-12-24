use std::collections::HashSet;
use crate::utils::grid::Grid;

pub(crate) fn run(test: bool) {
    let real_input = include_str!("input.txt");
    let test_input = include_str!("test.txt");
    let input = if test { test_input } else { real_input };

    let map = Grid::from(input);

    // group the plots together
    let mut plots: Vec<HashSet<(usize, usize)>> = Vec::new();
    for row in 0..map.get_height() {
        for col in 0..map.get_width() {
            // println!("Looking for plots that contain ({}, {}) with plant {}", row, col, map.get(row, col).unwrap());
            // if cell is already in a plot, skip it
            if plots.iter().any(|plot| plot.contains(&(row, col))) {
                continue;
            }
            let c = map.get(row, col);
            if c.is_some() {
                let mut partial_plot = HashSet::new();
                // recursively find all the connected plant spaces
                find_plot(row, col, c.unwrap(), &map, &mut partial_plot);

                // if any of the cells in this plot are already in another plot, merge them
                let mut plot_to_merge: Option<&mut HashSet<(usize, usize)>> = None;
                plots.iter_mut().any(|plot| {
                    if partial_plot.iter().any(|cell| plot.contains(cell)) {
                        plot_to_merge = Some(plot);
                        true
                    } else {
                        false
                    }
                });
                if let Some(ptm) = plot_to_merge.as_mut() {
                    ptm.extend(partial_plot);
                } else {
                    // add the plot to the list of plots
                    plots.push(partial_plot);
                }

            }
        }
    }

    let mut perimeter_counts = Vec::new();
    for plot in &plots {
        let mut perimeter = 0;
        for (row, col) in plot {
            let plant = map.get(*row, *col).unwrap();
            let (left, right, up, down) = map.get_adjacent(*row, *col);
            for direction in vec![left, right, up, down] {
                match direction {
                    Some(c) if c != plant => perimeter += 1,
                    None => perimeter += 1,
                    _ => (),
                }
            }
        }
        perimeter_counts.push(perimeter);
    }

    let mut total_fence_cost = 0u32;
    for p in plots.iter().zip(perimeter_counts.iter()) {
        let first = p.0.iter().next().unwrap();
        println!("{} plot is size: {}, perimeter {}", map.get(first.0, first.1).unwrap(), p.0.len(), p.1);

        /** Print the plots to look for ones that should connect but don't
        print the plots, for every plot we are going to print the whole map and if the point is in the plot, we print it
        if it is not in the plot but matches the letter, print the letter in lowercase
        otherwise, print .
        for row in 0..map.get_height() {
            for col in 0..map.get_width() {
                let c = map.get(row, col).unwrap();
                if p.0.contains(&(row, col)) {
                    print!("{}", c);
                } else if c == map.get(first.0, first.1).unwrap() {
                    print!("{}", c.to_lowercase());
                } else {
                    print!(".");
                }
            }
            println!();
        }
        */

        // look for plots points that are contained in more than one plot
        for other_plot in plots.iter().filter(|plot| *plot != p.0) {
            let intersection: HashSet<_> = p.0.intersection(other_plot).collect();
            if !intersection.is_empty() {
                println!("Found intersection of size {} between plots of size {} and {}", intersection.len(), p.0.len(), other_plot.len());
            }
        }

        // cost is area * perimeter
        total_fence_cost += (p.0.len() as u32) * p.1;
    }
    println!("Total fence cost: {}", total_fence_cost);
}

fn find_plot(row: usize, col: usize, plant: char, map: &Grid<char>, plot_points: &mut HashSet<(usize, usize)>) {
    // base case, the cell does not match
    if map.get(row, col) != Some(plant) {
        return;
    }
    plot_points.insert((row, col));
    // so we don't have infinite recursion, only check down and right
    let right = map.get_right(row, col);
    if right.is_some() {
        // println!("Checking right: {}", right.unwrap());
    }
    match right {
        Some(r) if r == plant => find_plot(row, col + 1, plant, map, plot_points),
        _ => (),
    }
    let down = map.get_bottom(row, col);
    if down.is_some() {
        // println!("Checking down: {}", down.unwrap());
    }
    match down {
        Some(d) if d == plant => find_plot(row + 1, col, plant, map, plot_points),
        _ => (),
    }

}