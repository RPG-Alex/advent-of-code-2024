use std::{fs::File, io::Read};

fn main() {
    let file = File::open("inputs/day4_1");
    let mut content = String::new();
    let _ = file
        .expect("failed to read file")
        .read_to_string(&mut content);
    let part1 = part_1(content.clone());
    println!("{}", part1);

    let part2 = part_2(content);
    println!("{}", part2);
}

fn part_1(content: String) -> i32 {
    let mut total = 0;
    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let directions: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let word = "XMAS";
    let word_chars: Vec<char> = word.chars().collect();
    let word_len = word_chars.len() as isize;

    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    for i in 0..height {
        for j in 0..width {
            for (dx, dy) in directions {
                let mut match_found = true;
                for k in 0..word_len {
                    let new_i = i + dx * k;
                    let new_j = j + dy * k;

                    if new_i < 0 || new_j < 0 || new_i >= height || new_j >= width {
                        match_found = false;
                        break;
                    }

                    if grid[new_i as usize][new_j as usize] != word_chars[k as usize] {
                        match_found = false;
                        break;
                    }
                }
                if match_found {
                    total += 1;
                }
            }
        }
    }

    total
}

fn part_2(content: String) -> i32 {
    let mut total = 0;

    total
}
