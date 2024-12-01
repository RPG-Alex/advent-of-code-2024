use std::{fs::File, io::Read};

fn main() {
    part_1();
}

fn part_1() {
    let file = File::open("inputs/day1_1");
    let mut content = String::new();
    let _ = file
        .expect("faled to read file")
        .read_to_string(&mut content);
    let mut total = 0;
    for mut line in content.split("\n") {
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

            println!("{:?}", list2);
        }
    }
}
