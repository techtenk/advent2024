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
            // make a plot for each square with it and it's adjacent
            let plant = map.get(row, col).unwrap();
            let mut miniset = HashSet::new();
            miniset.insert((row, col));
            let (left, right, up, down) = map.get_adjacent(row, col);
            if left.is_some() && left.unwrap() == plant {
                miniset.insert((row, col - 1));
            }
            if right.is_some() && right.unwrap() == plant {
                miniset.insert((row, col + 1));
            }
            if up.is_some() && up.unwrap() == plant {
                miniset.insert((row - 1, col));
            }
            if down.is_some() && down.unwrap() == plant {
                miniset.insert((row + 1, col));
            }

            plots.push(miniset);
        }
    }

    // for every plot in the list, check it for intersections against all other plots
    let mut still_to_merge = plots;
    let mut plots = Vec::new();
    println!("Merging plots");
    'checkplots: while let Some(mut i) = still_to_merge.pop() {
        for j in 0..still_to_merge.len() {
            let intersection: HashSet<_> = i.intersection(&still_to_merge[j]).collect();

            if !intersection.is_empty() {
                // merge the two plots
                i.extend(still_to_merge[j].iter());

                // println!("Merged plots {} and {} into {:?}", i, j, new_plot);
                still_to_merge.remove(j);

                still_to_merge.push(i);
                continue 'checkplots;
            }
        }
        plots.push(i);
    }
    println!("No more intersecting plots found");
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
        // println!("{} plot is size: {}, perimeter {}", map.get(first.0, first.1).unwrap(), p.0.len(), p.1);

        /* Print the plots to look for ones that should connect but don't
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

    // part 2 we count it differently, I think it's just counting the "corners" of the plots
    let mut total_fence_cost_discounted = 0u32;
    for plot in plots {
        let plant = map.get(plot.iter().next().unwrap().0, plot.iter().next().unwrap().1).unwrap();
        let plotsize = plot.len() as u32;
        let mut corners = 0;
        for point in plot {
            let plant = map.get(point.0, point.1).unwrap();
            let (left, right, up, down) = map.get_adjacent(point.0, point.1);

            let topleftcorner = match (left, up) {
                (Some(c), Some(d)) if c != plant && d != plant => true,
                (Some(c), Some(d)) if c == plant && d == plant => map.get(point.0 - 1, point.1 - 1).unwrap() != plant,
                (Some(c), None) if c != plant => true,
                (None, Some(d)) if d != plant => true,
                (None, None) => true,
                _ => false,
            };
            if topleftcorner {
                corners += 1;
            }
            let toprightcorner = match (right, up) {
                (Some(c), Some(d)) if c != plant && d != plant => true,
                (Some(c), Some(d)) if c == plant && d == plant => map.get(point.0 - 1, point.1 + 1).unwrap() != plant,
                (Some(c), None) if c != plant => true,
                (None, Some(d)) if d != plant => true,
                (None, None) => true,
                _ => false,
            };
            if toprightcorner {
                corners += 1;
            }
            let bottomleftcorner = match (left, down) {
                (Some(c), Some(d)) if c != plant && d != plant => true,
                (Some(c), Some(d)) if c == plant && d == plant => map.get(point.0 + 1, point.1 - 1).unwrap() != plant,
                (Some(c), None) if c != plant => true,
                (None, Some(d)) if d != plant => true,
                (None, None) => true,
                _ => false,
            };
            if bottomleftcorner {
                corners += 1;
            }
            let bottomrightcorner = match (right, down) {
                (Some(c), Some(d)) if c != plant && d != plant => true,
                (Some(c), Some(d)) if c == plant && d == plant => map.get(point.0 + 1, point.1 + 1).unwrap() != plant,
                (Some(c), None) if c != plant => true,
                (None, Some(d)) if d != plant => true,
                (None, None) => true,
                _ => false,
            };
            if bottomrightcorner {
                corners += 1;
            }


        }
        println!("{} plot is size: {}, corners: {}", plant, plotsize, corners);
        total_fence_cost_discounted += corners * plotsize;
    }
    println!("Total fence cost discounted: {}", total_fence_cost_discounted);
}
