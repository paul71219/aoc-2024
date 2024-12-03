use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

fn main() {
    // let in_filename = "/Users/pauld/w/aoc-2024/aoc_2024_pauldf/src/bin/day-1a/sample.txt";
    let in_filename = "/Users/pauld/w/aoc-2024/aoc_2024_pauldf/src/bin/day-1a/input.txt";

    let input = fs::read_to_string(in_filename).expect("failed to read fine");
    let in_lines = input.split('\n');

    let mut group1 = Vec::new();
    let mut group2 = Vec::new();

    for line in in_lines {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() == 2 {
            group1.push(i32::from_str(fields[0]).unwrap());
            group2.push(i32::from_str(fields[1]).unwrap());
        }
    }

    let d = score_b(group1, group2);

    println!("{}", d);
}

fn score_a(mut group1: Vec<i32>, mut group2: Vec<i32>) -> i32 {
    let mut d = 0;

    group1.sort();
    group2.sort();

    for row in group1.iter().zip(group2.iter()) {
        let (a, b) = row;
        d += (a - b).abs();
    }

    d
}

fn score_b(group1: Vec<i32>, group2: Vec<i32>) -> i32 {
    let mut d = 0;
    let group1 = count_occurrences(group1);
    let group2 = count_occurrences(group2);

    for (k1, v1) in group1 {
        d += k1 * v1 * group2.get(&k1).unwrap_or(&0);
    }

    d
}

fn count_occurrences(v: Vec<i32>) -> HashMap<i32, i32> {
    let mut c = HashMap::new();

    for i in v {
        c.entry(i).and_modify(|n| *n += 1).or_insert(1);
    }

    c
}
