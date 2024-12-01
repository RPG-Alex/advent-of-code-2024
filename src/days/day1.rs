use std::{fs::File, io::Read};

fn main() {
    let part1 = part_1();
    println!("{}", part1);
}

fn part_1() -> i32 {
    let file = File::open("inputs/day1_1");
    let mut content = String::new();
    let _ = file
        .expect("faled to read file")
        .read_to_string(&mut content);
    let mut total = 0;
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    for line in content.split("\n") {
        let mut line = line.split_ascii_whitespace().peekable();
        if line.peek().is_some() {
            let mut list1: Vec<i32> = line
                .next()
                .unwrap()
                .chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as i32))
                .collect();
            let mut list2: Vec<i32> = line
                .next()
                .unwrap()
                .chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as i32))
                .collect();
            list1.sort();
            list2.sort();
            for (i, id) in list1.iter().enumerate() {
                println!("{}", list2[i]);
                if id >= &list2[i] {
                    total += list1[i] - list2[i];
                } else {
                    total += list2[i] - list1[i];
                }
            }
        }
    }
    return total;
}
