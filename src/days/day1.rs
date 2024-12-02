use std::{fs::File, io::Read};
 
fn main() {
    let part1 = part_1();
    println!("part 1: {}\n", part1);
    let part2 = part_2();
    println!("part 2: {}", part2);
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
            let column1: i32 = line.next().unwrap().parse::<i32>().unwrap();
            let column2: i32 = line.next().unwrap().parse::<i32>().unwrap();
            list1.push(column1);
            list2.push(column2);
        }
    }
    list1.sort();
    list2.sort();
    for (i, id) in list1.iter().enumerate() {
        let mut result = id - list2[i];
        if result < 0 {
            result = result * -1;
        }
        total += result;
    }
    return total;
}

fn part_2() -> i32 {
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
            let column1: i32 = line.next().unwrap().parse::<i32>().unwrap();
            let column2: i32 = line.next().unwrap().parse::<i32>().unwrap();
            list1.push(column1);
            list2.push(column2);
        }
    }

    list1.sort();
    list2.sort();
    for (i, id) in list1.iter().enumerate() {
        let found = list2.iter().find(|&x| x == id);
        if found.is_some() {
            let mut sub = 0;
            for (j, jid) in list2.iter().enumerate() {
                if jid == id {
                    sub += 1;
                }
            }
            total += sub * id;
        }
    }
    total

}
