use std::{fs::File, i64::MIN, io::Read};

fn main() {
    let file = File::open("inputs/day7_1");
    let mut content = String::new();
    let _ = file
        .expect("failed to read file")
        .read_to_string(&mut content);
    let calibrations: Vec<&str> = content.lines().collect();
    let part1 = part_1(calibrations.clone());
    println!("{}", part1);

    let part2 = part_2(calibrations);
    println!("{}", part2);
}

fn part_1(calibrations: Vec<&str>) -> i64 {
    let mut total = 0;
    for calibration in calibrations.iter() {
        let mut equation: Vec<i64> = Vec::new();
        let mut result: i64 = MIN;
        for (i, part) in calibration.split(':').into_iter().enumerate() {
            if i == 0 {
                result = part.parse::<i64>().unwrap();
            } else {
                equation = part
                    .split_whitespace()
                    .map(|c| c.parse::<i64>().unwrap())
                    .collect();
            }
        }
        let operator_spots = (equation.len() - 1) as u32;
        let possibilities = 2_usize.pow(operator_spots);
        'outer: for i in 0..possibilities {
            let mut current_value = equation[0];
            for pos in 0..operator_spots {
                let bit = (i >> pos) & 1;
                let next_value = equation[(pos + 1) as usize];
                if bit == 0 {
                    current_value = current_value + next_value;
                } else {
                    current_value = current_value * next_value;
                }
            }

            if current_value == result {
                total += result;
                break 'outer;
            }
        }
    }

    total
}

fn part_2(equations: Vec<&str>) -> i32 {
    let mut total = 0;

    total
}
