use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

fn main() {
    let file = File::open("inputs/day8_1");
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
fn part_1(grid_rows: Vec<&str>) -> usize {
    let width = grid_rows[0].len();
    let height = grid_rows.len();

    let mut antenna_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (y, row) in grid_rows.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c != '.' {
                antenna_map.entry(c).or_insert_with(Vec::new).push((x, y));
            }
        }
    }

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for (_, positions) in &antenna_map {
        for (i, &(x1, y1)) in positions.iter().enumerate() {
            for &(x2, y2) in &positions[i + 1..] {
                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;
                let before_node = (x1 as isize - dx, y1 as isize - dy);
                let after_node = (x2 as isize + dx, y2 as isize + dy);

                if is_within_bounds(before_node, width, height) {
                    antinodes.insert((before_node.0 as usize, before_node.1 as usize));
                }
                if is_within_bounds(after_node, width, height) {
                    antinodes.insert((after_node.0 as usize, after_node.1 as usize));
                }
            }
        }
    }
    antinodes.len()
}

fn is_within_bounds(coord: (isize, isize), width: usize, height: usize) -> bool {
    coord.0 >= 0 && coord.1 >= 0 && coord.0 < width as isize && coord.1 < height as isize
}

fn part_2(grid_rows: Vec<&str>) -> i64 {
    let mut total = 0;

    total
}
