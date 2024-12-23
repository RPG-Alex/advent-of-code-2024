use std::{fs::File, io::Read, usize};

fn main() {
    let file = File::open("inputs/day9_1");
    let mut content = String::new();
    let _ = file
        .expect("failed to read file")
        .read_to_string(&mut content);
    let disk_items: Vec<usize> = content
        .lines()
        .next()
        .unwrap()
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let part1 = part_1(disk_items.clone());
    println!("{}", part1);

    let part2 = part_2(disk_items);
    println!("{}", part2);
}

fn part_1(disk_map: Vec<usize>) -> i64 {
    let mut total = 0;
    let mut disk: Vec<usize> = Vec::new();
    let mut bid: usize = 0;

    for (id, block) in disk_map.iter().enumerate() {
        if id % 2 != 0 {
            for i in 0..block.clone() {
                disk.push(usize::MAX);
            }
        } else {
            for i in 0..block.clone() {
                disk.push(bid);
            }
            bid += 1;
        }
    }
    let mut right: usize = disk.len() - 1;

    for left in 0..disk.len() {
        while disk[left] == usize::MAX {
            if right <= left {
                break;
            }
            if disk[right] == usize::MAX {
                right -= 1;
            } else if disk[left] != usize::MAX && disk[right] != usize::MAX {
                right -= 1;
            } else {
                disk[left] = disk[right];
                disk[right] = usize::MAX;
                right -= 1;
            }
        }
    }

    for (id, block) in disk.iter().enumerate() {
        if block == &usize::MAX {
            break;
        } else {
            total += (id as i64) * (block.clone() as i64);
        }
    }

    total
}

fn part_2(disk_map: Vec<usize>) -> i32 {
    let mut total = 0;

    total
}
