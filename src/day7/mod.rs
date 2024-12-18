
enum OPERATION {
    Add,
    Multiply,
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
        let mut counter = 0; // we can mask this so that we get every combination of operations
        let mut finished = true;

        let line_valid = loop {
            let mut mask = 1;

            let rightside = numbers.iter().copied().reduce(|acc, x| {
                // println!("{} {}", counter, mask);
                let operation = match counter & mask {
                    0 => { finished = false; OPERATION::Add },
                    1.. => OPERATION::Multiply,
                    _ => panic!("Invalid operation"),
                };
                mask  = if mask == 0 { 1 } else { mask * 2 }; // multiply by 2 unless it is 0
                match operation {
                    OPERATION::Add => { /*println!("{} + {}", acc, x);*/ acc + x },
                    OPERATION::Multiply => { /*println!("{} * {}", acc, x);*/ acc * x },
                }
            });
            if rightside.unwrap() == result {
                break true;
            }
            if finished {
                break false;
            }
            counter += 1;
            finished = true;

        };

        if line_valid {
            valid_lines.push(result);
        }
    }
    println!("Total Calibration Result: {}", valid_lines.iter().sum::<u64>());
}