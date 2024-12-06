use std::{fs::File, io::Read, ptr::null, usize::MAX};

fn main() {
    let file = File::open("inputs/day6_1");
    let mut content = String::new();
    let _ = file
        .expect("failed to read file")
        .read_to_string(&mut content);
    let part1 = part_1(content.clone());
    println!("{}", part1);

    let part2 = part_2(content);
    println!("{}", part2);
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Gone,
}

fn part_1(content: String) -> i32 {
    let mut total = 0;
    let mut direction = Direction::Up;

    let mut grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    let mut guard_loc: (usize, usize) = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&n| n == '^').map(|j| (i, j)))
        .unwrap();

    while direction != Direction::Gone {
        if grid[guard_loc.0][guard_loc.1] != 'X' {
            grid[guard_loc.0][guard_loc.1] = 'X';
            total += 1;
        }
        match direction {
            Direction::Up => {
                if guard_loc.0 == 0 {
                    direction = Direction::Gone;
                } else if grid[guard_loc.0 - 1][guard_loc.1] == '#' {
                    direction = Direction::Right;
                } else {
                    guard_loc = (guard_loc.0 - 1, guard_loc.1);
                }
            }
            Direction::Down => {
                if guard_loc.0 == grid.len() - 1 {
                    direction = Direction::Gone;
                } else if grid[guard_loc.0 + 1][guard_loc.1] == '#' {
                    direction = Direction::Left;
                } else {
                    guard_loc = (guard_loc.0 + 1, guard_loc.1);
                }
            }
            Direction::Left => {
                if guard_loc.1 == 0 {
                    direction = Direction::Gone;
                } else if grid[guard_loc.0][guard_loc.1 - 1] == '#' {
                    direction = Direction::Up
                } else {
                    guard_loc = (guard_loc.0, guard_loc.1 - 1);
                }
            }
            Direction::Right => {
                if guard_loc.1 == grid[guard_loc.0].len() - 1 {
                    direction = Direction::Gone;
                } else if grid[guard_loc.0][guard_loc.1 + 1] == '#' {
                    direction = Direction::Down
                } else {
                    guard_loc = (guard_loc.0, guard_loc.1 + 1);
                }
            }
            _ => break,
        }
    }

    total
}

fn part_2(content: String) -> i32 {
    let mut total = 0;

    total
}
