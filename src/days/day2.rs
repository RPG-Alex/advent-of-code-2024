use std::{fs::File, io::Read};

fn main() {
    let part_1 = part_1();
    println!("{}", part_1);
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
