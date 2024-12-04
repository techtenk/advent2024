use std::collections::HashMap;
use std::io::BufRead;
use std::iter::Map;

pub(crate) fn run() {
    let input = include_bytes!("input.txt");
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in input.lines() {
        let li = line.unwrap();
        let mut l = li.split_whitespace();
        list1.push(l.next().unwrap().parse::<i32>().unwrap());
        list2.push(l.next().unwrap().parse::<i32>().unwrap());
    }
    list1.sort();
    list2.sort();
    let mut total = 0;
    for item in list1.iter().zip(list2.iter()) {
        total += (item.0 - item.1).abs();
    }
    println!("Total difference between lists is {}", total);

    // make a map out of list2
    let mut listmap: HashMap<i32, i32> = HashMap::new();
    for item in list2.iter() {
        let mut entry = listmap.entry(*item).or_insert(0);
        *entry += 1;
    }
    // now total up the similarly score
    let mut score = 0;
    for item in list1.iter() {
        score += item * *listmap.get(item).unwrap_or(&0);
    }

    println!("Total similarity score is {}", score);
}