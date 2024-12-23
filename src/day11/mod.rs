use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Stone {
    mark: u64
}

impl Stone {
    fn new(mark: u64) -> Self {
        Self {
            mark
        }
    }

    fn get_mark(&self) -> u64 {
        self.mark
    }

}

impl StonesArrangement for Stone {
    fn get_length(&self) -> usize {
        1
    }

    fn get_first_stone(&self) -> Option<&Stone> {
        Some(self)
    }

    fn get_next_blink(&self) -> Box<dyn StonesArrangement> {
        // if mark is 0, return Stone::new(1)
        // if mark has even number of digits, return new Vec<Stone> with 2 stones
        // else return Stone::new(mark * 2024)
        if self.mark == 0 {
            Box::new(Stone::new(1))
        } else if self.mark.to_string().len() % 2 == 0 {
            let mark_str = self.mark.to_string();
            let first = mark_str[..mark_str.len() / 2].parse::<u64>().unwrap();
            let second = mark_str[mark_str.len() / 2..].parse::<u64>().unwrap();
            Box::new(vec![Stone::new(first), Stone::new(second)])
        } else {
            Box::new(Stone::new(self.mark * 2024))
        }
    }

    fn get_stones(&self) -> Vec<Stone> {
        vec![*self]
    }
}

impl StonesArrangement for Vec<Stone> {
    fn get_length(&self) -> usize {
        self.len()
    }

    fn get_first_stone(&self) -> Option<&Stone> {
        self.first()
    }

    fn get_next_blink(&self) -> Box<dyn StonesArrangement> {
        let mut new_stones = Vec::new();
        for stone in self.iter() {
            let after_blink = stone.get_next_blink();
            new_stones.extend(after_blink.get_stones());
        }
        Box::new(new_stones)
    }

    fn get_stones(&self) -> Vec<Stone> {
        self.iter().copied().collect()
    }
}
trait StonesArrangement {
    fn get_length(&self) -> usize;
    fn get_first_stone(&self) -> Option<&Stone>;
    fn get_next_blink(&self) -> Box<dyn StonesArrangement>;

    fn get_stones(&self) -> Vec<Stone>;
}
pub(crate) fn run(test: bool) {
    let real_input = include_str!("input.txt");
    let test_input = include_str!("test.txt");
    let input = if test { test_input } else { real_input };

    {
        let mut stones = Vec::new();
        for line in input.lines() {
            let marks = line.split_whitespace();
            for mark in marks {
                let mark = mark.parse::<u64>().unwrap();
                stones.push(Stone::new(mark));
            }
        }

        // blink 25 times
        for _ in 0..25 {
            let mut new_stones = Vec::new();
            for stone in stones.iter() {
                let after_blink = stone.get_next_blink();
                new_stones.extend(after_blink.get_stones());
            }
            stones = new_stones;
        }
        println!("Number of stones after 25 blinks: {}", stones.len());
    } //end part 1

    // 75 times is too much to keep a list of all of them
    let mut stones2 = Vec::new();
    for line in input.lines() {
        let marks = line.split_whitespace();
        for mark in marks {
            let mark = mark.parse::<u64>().unwrap();
            stones2.push(Stone::new(mark));
        }
    }
    let mut stone_map = HashMap::new();
    for stone in stones2.iter() {
        let mark = stone.get_mark();
        let count = stone_map.entry(mark).or_insert(0);
        *count += 1;
    }

    for _ in 0..75 {
        let mut step_map = HashMap::new();
        for entry in stone_map.iter_mut() {
            let after_blink = Stone::new(*entry.0).get_next_blink();
            let stones = after_blink.get_stones();
            for stone in stones.iter() {
                let mark = stone.get_mark();
                let count = step_map.entry(mark).or_insert(0);
                *count += *entry.1;
            }
            *entry.1 = 0;
        }
        // merge the step map into the stone map
        for entry in step_map.iter() {
            let count = stone_map.entry(*entry.0).or_insert(0);
            *count += *entry.1;
        }
    }

    let mut total75 = 0usize;
    for entry in stone_map.iter() {
        total75 += entry.1;
    }
    println!("Total number of stones after 75 blinks: {}", total75);
}