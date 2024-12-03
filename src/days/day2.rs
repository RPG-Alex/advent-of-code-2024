use std::{fs::File, io::Read};

fn main() {
    let part_1 = part_1();
    println!("part 1: {}", part_1);

    let part_2 = part_2();
    println!("part 2: {}", part_2);
}

fn part_1() -> i32 {
    let file = File::open("inputs/day2_1");
    let mut content = String::new();
    let _ = file
        .expect("failed to read file")
        .read_to_string(&mut content);

    let mut safe = 0;

    for line in content.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .filter_map(|level| level.parse::<i32>().ok())
            .collect();

        let mut increasing = true;
        let mut is_safe = true;

        for i in 1..levels.len() {
            let diff = levels[i] - levels[i - 1];
            if diff > 3 || diff < -3 || diff == 0 {
                is_safe = false;
                break;
            }
            if i == 1 {
                increasing = diff > 0;
            } else if (diff > 0 && !increasing) || (diff < 0 && increasing) {
                is_safe = false;
            }
        }
        if is_safe {
            safe += 1;
        }
    }

    safe
}

fn part_2() -> i32 {
    let file = File::open("inputs/day2_1");
    let mut content = String::new();
    let _ = file
        .expect("failed to read file")
        .read_to_string(&mut content);

    let mut safe = 0;

    for line in content.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .filter_map(|level| level.parse::<i32>().ok())
            .collect();

        if is_safe(&levels) {
            safe += 1;
        } else {
            for i in 0..levels.len() {
                let mut modified_levels = levels.clone();
                modified_levels.remove(i);
                if is_safe(&modified_levels) {
                    safe += 1;
                    break;
                }
            }
        }
    }

    safe
}

fn is_safe(levels: &[i32]) -> bool {
    let first_diff = levels[1] - levels[0];
    if first_diff == 0 || first_diff > 3 || first_diff < -3 {
        return false;
    }

    let increasing = first_diff > 0;

    for i in 2..levels.len() {
        let diff = levels[i] - levels[i - 1];
        if diff == 0 || diff > 3 || diff < -3 {
            return false;
        }
        if (diff > 0 && !increasing) || (diff < 0 && increasing) {
            return false;
        }
    }
    true
}
