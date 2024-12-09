use std::collections::{HashMap, HashSet};
use std::{fs, usize};
use std::io::Lines;
use std::str::FromStr;

fn main() {
    let in_filename = "/Users/pauld/w/aoc-2024/aoc_2024_pauldf/src/bin/day-5/sample.txt";

    let s = score_file(in_filename);
    if s != 123 {
        panic!("unexpected score {}", s);
    }
    println!("solved sample!");

    let in_filename = "/Users/pauld/w/aoc-2024/aoc_2024_pauldf/src/bin/day-5/input.txt";

    let s = score_file(in_filename);

    println!("{}", s);
}

fn score_file(filename: &str) -> i32 {
    let input = fs::read_to_string(filename).expect("failed to read fine");
    let mut lines = input.split('\n').into_iter();

    let mut order_rules: Vec<Vec<_>> = vec![];
    loop {
        let line = lines.next();
        match line {
            Some("") => break,
            Some(s) => order_rules.push(split_to_i32(s, "|")),
            None => panic!("EOF"),
        }
    }

    let updates: Vec<Vec<_>> = lines.filter(|ln| ! ln.is_empty()).map(|ln: &str| split_to_i32(ln, ",")).collect();
    println!("{} rules, {} updates", order_rules.len(), updates.len());
    score_b(order_rules, updates)
}


fn split_to_i32(line: &str, pat: &str) -> Vec<i32> {
    line.split(pat).flat_map(|s| str_i32(s)).collect()
}

fn s2cv(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn str_i32(s: &str) -> Option<i32> {
    match i32::from_str(s) {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}

fn score_a(order_rules: Vec<Vec<i32>>, updates: Vec<Vec<i32>>) -> i32 {
    let mut rules_map = HashMap::new();
    let empty_set = HashSet::new();
    let mut score = 0;
    for rule in order_rules {
        rules_map.entry(rule[0]).or_insert(HashSet::new()).insert(rule[1]);
    }

    for update in updates {
        if is_ordered(&update, &rules_map, &empty_set) {
            score += update[update.len() / 2];
        }
    }
    score
}

fn is_ordered(update: &Vec<i32>, rules_map: &HashMap<i32, HashSet<i32>>, empty_set: &HashSet<i32>) -> bool {
    let mut pages_seen = HashSet::new();
    for page in update.iter() {
        let possible_conflicts = rules_map.get(&page).unwrap_or(empty_set);
        let conflicts = possible_conflicts.intersection(&pages_seen);
        if conflicts.count() > 0 {
            return false;
        }
        pages_seen.insert(*page);
    }
    true
}

fn score_b(order_rules: Vec<Vec<i32>>, updates: Vec<Vec<i32>>) -> i32 {
    let mut rules_map = HashMap::new();
    let empty_set = HashSet::new();
    let mut score = 0;
    for rule in order_rules {
        rules_map.entry(rule[0]).or_insert(HashSet::new()).insert(rule[1]);
    }

    for update in updates {
        if is_ordered(&update, &rules_map, &empty_set) {
            continue;
        }
        let mut later_pages: HashSet<i32> = update.into_iter().collect();
        let mut ordered_pages = vec!();
        while ! later_pages.is_empty() {
            let blocked_pages: HashSet<i32> = later_pages.iter().flat_map(|p| rules_map.get(p).unwrap_or(&empty_set)).map(|p| *p).collect();
            let mut candidate_pages: Vec<i32> = later_pages.difference(&blocked_pages).into_iter().map(|p| *p).collect();
            later_pages = later_pages.intersection(&blocked_pages).into_iter().map(|p| *p).collect();
            ordered_pages.append(&mut candidate_pages);
        }

        score += ordered_pages[ordered_pages.len() / 2];
    }
    score
}

fn get_at (lines: Vec<Vec<char>>, x:usize, y:usize, dx:isize, dy:isize) -> Option<char> {
    lines.get((x as isize + dx) as usize)
                    .and_then(|l| l.get((y as isize + dy) as usize))
                    .and_then(|c| Some(*c))
}

fn count_occurrences(v: Vec<i32>) -> HashMap<i32, i32> {
    let mut c = HashMap::new();

    for i in v {
        c.entry(i).and_modify(|n| *n += 1).or_insert(1);
    }

    c
}
