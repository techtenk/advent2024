use regex::Regex;
pub(crate) fn run() {
    let input = include_bytes!("input.txt");
    // part 1
    let str_input = input.iter().map(|&c| c as char).collect::<String>();
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut total = 0;
    for m in re.captures_iter(&str_input) {
        total += m[1].parse::<i32>().unwrap() * m[2].parse::<i32>().unwrap();
    }
    println!("result part 1: {}", total);

    // part 2
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut total = 0;
    let mut doing = true;
    for m in re.captures_iter(&str_input) {
        if m[0].eq("do()") {
            doing = true;
            continue;
        } else if m[0].eq("don't()") {
            doing = false;
            continue;
        }
        if doing {
            total += m[1].parse::<i32>().unwrap() * m[2].parse::<i32>().unwrap();
        }
    }
    println!("result part 2: {}", total);
}