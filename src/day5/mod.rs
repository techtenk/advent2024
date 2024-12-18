use std::io::BufRead;

#[derive(PartialEq)]
struct Rule {
    first: u32,
    second: u32,
}
pub(crate) fn run() {
    let input = include_bytes!("input.txt");
    let mut rules = Vec::new();
    let mut updates = Vec::new();

    let mut updates_mode = false;
    // collect the rules and updates
    for line in input.lines() {

        let line = line.unwrap();
        if line.is_empty() {
            updates_mode = true;
            continue;
        }

        match updates_mode {
            false => rules.push(parse_rule(&line)),
            true => {
                updates.push(line.split(',').map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>());
            },
        }
    }

    let mut valid_updates = Vec::new();
    let mut invalid_updates = Vec::new();
    'update: for update in updates {
        let mut inverse_rules = Vec::new();
        for (i, x) in update.iter().enumerate() {
            for y in &update[(i + 1)..] {
                inverse_rules.push(Rule { first: *y, second: *x});
            }
        }
        // check if any of the inverse rules are in the rules
        for rule in &inverse_rules {
            if rules.contains(rule) {
                invalid_updates.push(update);
                continue 'update;
            }
        }
        // if none of the inverse rules are in the rules, then the update is valid
        valid_updates.push(update);
    }
    // println!("{:?}", valid_updates);
    let total = valid_updates.iter().fold(0, |acc, x| {
        let middle_index = (x.len() - 1) / 2;
        acc + x[middle_index]
    });

    println!("result part 1: {}", total);

    // continue to part 2
    let custom_sort = |a: &u32, b: &u32| {
        if rules.contains(&Rule { first: *a, second: *b }) {
            return std::cmp::Ordering::Less;
        } else if rules.contains(&Rule { first: *b, second: *a }) {
            return std::cmp::Ordering::Greater;
        }
        return std::cmp::Ordering::Equal;
    };


    for update in &mut invalid_updates {
        update.sort_by(custom_sort);
    }

    let total = invalid_updates.iter().fold(0, |acc, x| {
        let middle_index = (x.len() - 1) / 2;
        acc + x[middle_index]
    });
    println!("result part 2: {}", total);
}

fn parse_rule(line: &str) -> Rule {
    let mut parts = line.split("|");
    let first = parts.next().unwrap().parse::<u32>().unwrap();
    let second = parts.next().unwrap().parse::<u32>().unwrap();
    Rule { first, second }
}