
#[derive(PartialEq, Copy, Clone)]
enum OPERATION {
    Add,
    Multiply,
    Concat,
}
pub(crate) fn run(test: bool) {
    let real_input = include_str!("input.txt");
    let test_input = include_str!("test.txt");
    let input = if test { test_input } else { real_input };

    let mut valid_lines = Vec::new();

    for line in input.lines() {
        // tokenize the line
        let mut tokens = line.split_whitespace();
        let result = tokens.next().unwrap().split(':').next().unwrap().parse::<u64>().unwrap();
        let numbers = tokens.map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        let mut operation_queue = Vec::new();
        let mut finished = true;

        let line_valid = loop {
            let mut mask = 1;
            let mut op_queue_copy = operation_queue.clone();

            let rightside = numbers.iter().copied().reduce(|acc, x| {
                // println!("{} {}", counter, mask);
                let operation = op_queue_copy.pop();
                if operation.is_none() {
                    operation_queue.push(OPERATION::Add);
                }
                if operation.is_none() || operation != Some(OPERATION::Concat) {
                    finished = false;
                }
                mask  = if mask == 0 { 1 } else { mask * 2 }; // multiply by 2 unless it is 0
                match operation {
                    Some(OPERATION::Add)|None => { /*println!("{} + {}", acc, x);*/ acc + x },
                    Some(OPERATION::Multiply) => { /*println!("{} * {}", acc, x);*/ acc * x },
                    Some(OPERATION::Concat) => {
                        /*println!("{} * {}", acc, x);*/
                        let new_acc = acc.to_string() + &x.to_string();
                        new_acc.parse().unwrap()
                    }
                }
            });
            if rightside.unwrap() == result {
                break true;
            }
            if finished {
                break false;
            }
            // increment the operations queue by changing the first operation to the next one and cascading the changes
            increment(&mut operation_queue);
            finished = true;

        };

        if line_valid {
            valid_lines.push(result);
        }
    }
    println!("Total Calibration Result: {}", valid_lines.iter().sum::<u64>());
}

fn increment(queue: &mut Vec<OPERATION>) {
    // change the first operation to the next one, and continue to next digit if the operation is Concat
    for operation in queue.iter_mut() {
        match operation {
            OPERATION::Add => {
                *operation = OPERATION::Multiply;
                return;
            },
            OPERATION::Multiply => {
                *operation = OPERATION::Concat;
                return;
            },
            OPERATION::Concat => {
                *operation = OPERATION::Add;
            },
        }
    }
}