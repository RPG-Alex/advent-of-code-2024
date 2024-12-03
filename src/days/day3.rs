use regex::Regex;
use std::{fs::File, io::Read};

fn main() {
    let file = File::open("inputs/day3_1");
    let mut content = String::new();
    let _ = file
        .expect("failed to read file")
        .read_to_string(&mut content);
    let part1 = part_1(content);
    println!("{}", part1)
}
fn part_1(content: String) -> i32 {
    let mut total = 0;
    let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    for matches in regex.find_iter(content.as_str()) {
        let mut nums = matches.as_str().to_string();
        nums.retain(|c| c != 'm' && c != 'u' && c != 'l' && c != '(' && c != ')');
        //        println!("{}", nums);
        if nums.len() > 1 {
            let mut mult = 1;
            for num in nums.split(",").into_iter() {
                let num = num.parse::<i32>().unwrap();
                mult = mult * num;
            }
            total += mult;
        }
    }
    total
}
