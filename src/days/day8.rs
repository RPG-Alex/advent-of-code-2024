use std::{fs::File, io::Read};

fn main() {
    let file = File::open("inputs/day8_sample");
    let mut content = String::new();
    let _ = file
        .expect("failed to read file")
        .read_to_string(&mut content);
    let grid_rows: Vec<&str> = content.lines().collect();
    let part1 = part_1(grid_rows.clone());
    println!("{}", part1);

    let part2 = part_2(grid_rows);
    println!("{}", part2);
}

fn part_1(grid_rows: Vec<&str>) -> i64 {
    let mut total = 0;
    let mut antenna: Vec<char>;
    let mut antenna: Vec<char, (usize, usize)>;

    for (i, row) in grid_rows.iter().enumerate() {
        for (j, c) in row.chars().into_iter().enumerate() {
            if !antenna.contains(&c) && c != '.' {
                antenna.push(c);
            }
        }
    }
    println!("{:?}", antenna);
    total
}

fn part_2(grid_rows: Vec<&str>) -> i64 {
    let mut total = 0;

    total
}
