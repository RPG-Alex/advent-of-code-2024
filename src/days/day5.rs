use std::{collections::HashMap, fs::File, io::Read};

#[derive(Debug)]
struct Parts {
    rule: String,
    pages: String,
}

fn main() {
    let file = File::open("inputs/day5_1");
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
    let rule_pages = rule_break(content);
    let rules: Vec<(i32, i32)> = rules_helper(rule_pages.rule);
    let pages_list: Vec<Vec<i32>> = pages_helper(rule_pages.pages);

    let mut adjacent_map: HashMap<i32, Vec<i32>> = HashMap::new();

    for (x, y) in rules.iter() {
        adjacent_map.entry(*x).or_insert_with(Vec::new).push(*y);
    }

    for pages in pages_list.iter() {
        let mut position: HashMap<i32, usize> = HashMap::new();
        for (i, &page) in pages.iter().enumerate() {
            position.insert(page, i);
        }
        let mut valid_change = true;

        for (&x, targets) in &adjacent_map {
            for &y in targets {
                if let (Some(pos_x), Some(pos_y)) = (position.get(&x), position.get(&y)) {
                    if pos_x >= pos_y {
                        valid_change = false;
                        break;
                    }
                }
            }

            if !valid_change {
                break;
            }
        }

        if valid_change {
            let mid_index = pages.len() / 2;
            let mid_page = pages[mid_index];
            total += mid_page;
        }
    }

    total
}

fn part_2(content: String) -> i32 {
    let mut total = 0;

    total
}

fn rules_helper(content: String) -> Vec<(i32, i32)> {
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let _: Vec<_> = content
        .lines()
        .map(|line| {
            if line.contains("|") {
                let mut line = line.split("|");
                rules.push((
                    line.next().unwrap().parse::<i32>().unwrap(),
                    line.next().unwrap().parse::<i32>().unwrap(),
                ));
            }
        })
        .collect();
    rules
}
fn pages_helper(content: String) -> Vec<Vec<i32>> {
    let mut pages: Vec<Vec<i32>> = Vec::new();
    let _: Vec<_> = content
        .lines()
        .map(|line| {
            if line.contains(",") {
                let line = line.split(",");
                let mut page_line: Vec<i32> = Vec::new();
                for p in line.into_iter() {
                    page_line.push(p.parse::<i32>().unwrap());
                }
                pages.push(page_line);
            }
        })
        .collect();
    pages
}

fn rule_break(content: String) -> Parts {
    let mut broken: Parts = Parts {
        pages: "".to_string(),
        rule: "".to_string(),
    };
    let lines: Vec<&str> = content.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if !line.contains("|") && !line.contains(",") {
            broken.rule = lines[0..i - 1].join("\n");
            broken.pages = lines[i + 1..].join("\n");
            break;
        }
    }
    broken
}
