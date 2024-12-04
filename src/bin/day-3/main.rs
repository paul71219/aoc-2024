use regex::Regex;
use std::fs;
use std::str::FromStr;

fn main() {
    let in_filename = "/Users/pauld/w/aoc-2024/aoc_2024_pauldf/src/bin/day-3/sample.txt";

    let s = score_file(in_filename);
    if s != 48 {
        panic!("unexpected score {}", s);
    }
    println!("solved sample!");

    let in_filename = "/Users/pauld/w/aoc-2024/aoc_2024_pauldf/src/bin/day-3/input.txt";

    let s = score_file(in_filename);

    println!("{}", s);
}

fn score_file(filename: &str) -> i32 {
    let input = fs::read_to_string(filename).expect("failed to read fine");
    score_b(&input)
}

fn str_i32(s: &str) -> Option<i32> {
    match i32::from_str(s) {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}

fn score_a(haystack: &str) -> i32 {
    let mut score: i32 = 0;

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    for (_, [n1, n2]) in re.captures_iter(haystack).map(|c| c.extract()) {
        match (str_i32(n1), str_i32(n2)) {
            (Some(i1), Some(i2)) => score += i1 * i2,
            _ => (),
        }
    }
    score
}

fn score_b(haystack: &str) -> i32 {
    let mut score: i32 = 0;
    let mut on = true;

    let re = Regex::new(r"(mul)\(([0-9]+),([0-9]+)\)|(do)()()\(\)|(don)(')(t)\(\)").unwrap();

    for (whole, [op, n1, n2]) in re.captures_iter(haystack).map(|c| c.extract()) {
        match (op, str_i32(n1), str_i32(n2)) {
            ("mul", Some(i1), Some(i2)) => {
                if on {
                    score += i1 * i2
                }
            }
            ("do", _, _) => on = true,
            ("don", _, _) => on = false,
            _ => panic!("unexpected match {}", whole),
        }
    }
    score
}
