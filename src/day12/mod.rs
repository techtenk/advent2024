use crate::utils::grid::Grid;

pub(crate) fn run(test: bool) {
    let real_input = include_str!("input.txt");
    let test_input = include_str!("test.txt");
    let input = if test { test_input } else { real_input };

    let map = Grid::from(input);

    // group the plots together
    let mut plots: Vec<Vec<(usize, usize)>> = Vec::new();
    for row in 0..map.get_height() {
        for col in 0..map.get_width() {
            // if cell is already in a plot, skip it
            if plots.iter().any(|plot| plot.contains(&(row, col))) {
                continue;
            }
            let c = map.get(row, col);
            if c.is_some() {
                // recursively find all the connected plant spaces
                let mut partial_plot: Vec<(usize, usize)> = find_plot(row, col, c.unwrap(), &map).unwrap();

                // at this point the plot may have duplicates and it may be incomplete
                partial_plot.sort();
                partial_plot.dedup();
                // if any of the cells in this plot are already in another plot, merge them
                let mut plot_to_merge: Option<&mut Vec<(usize, usize)>> = None;
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
                    ptm.sort();
                    ptm.dedup();
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
        println!("{} plot is size: {}, perimeter {}", map.get(p.0[0].0, p.0[0].1).unwrap(), p.0.len(), p.1);
        // cost is area * perimeter
        total_fence_cost += (p.0.len() as u32) * p.1;
    }
    println!("Total fence cost: {}", total_fence_cost);
}

fn find_plot(row: usize, col: usize, plant: char, map: &Grid<char>) -> Option<Vec<(usize, usize)>> {
    // base case, the cell does not match
    if map.get(row, col) != Some(plant) {
        return None
    }
    let mut plot = vec![(row, col)];
    // so we don't have infinite recursion, only check down and right
    let (_, right, _, down) = map.get_adjacent(row, col);
    match right {
        Some(r) if r == plant => plot.extend(find_plot(row, col + 1, plant, map)?),
        _ => (),
    }
    match down {
        Some(d) if d == plant => plot.extend(find_plot(row + 1, col, plant, map)?),
        _ => (),
    }
    Some(plot)
}