use std::{fs::File, io::Read};

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
    let mut rules: Vec<(i32, i32)> = rules_helper(rule_pages.rule);
    let mut pages: Vec<Vec<i32>> = pages_helper(rule_pages.pages);

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
                let mut line = line.split(",");
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

    for (i, line) in content.lines().into_iter().enumerate() {
        if !line.contains("|") && !line.contains(",") {
            broken.rule = content[0..i - 1].to_string();
            broken.pages = content[i + 1..content.len()].to_string();
            break;
        }
    }

    broken
}
