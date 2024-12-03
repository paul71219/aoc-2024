use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

fn main() {
    let in_filename = "/Users/pauld/w/aoc-2024/aoc_2024_pauldf/src/bin/day-2/sample.txt";

    
    let s = score_b(read_file(in_filename));
    if s != 4 {
        panic!("unexpected score {}", s);
    }
    println!("solved sample!");

    let in_filename = "/Users/pauld/w/aoc-2024/aoc_2024_pauldf/src/bin/day-2/input.txt";

    let s = score_b(read_file(in_filename));

    println!("{}", s);
}

fn read_file(filename: &str) -> Vec<Vec<i32>> {

    let input = fs::read_to_string(filename).expect("failed to read fine");
    let in_lines = input.split('\n');

    let mut reports = Vec::new();

    for line in in_lines {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() > 1 {
            reports.push(fields.iter().filter_map(str_i32).collect());
        }
    }

    reports
}

fn str_i32 (s: &&str) -> Option<i32> {
    match i32::from_str(s) {
        Ok(n) => Some(n),
        Err(_) => None
    }
}

fn score_a(reports: Vec<Vec<i32>>) -> i32 {
    let mut safe_count = 0;
    for report in reports {
        if report.len() > 1 {
            let found_unsafe = report_safe(&report);
            if ! found_unsafe {
                safe_count += 1;
            }
        }
    }
    safe_count
}

fn score_b(reports: Vec<Vec<i32>>) -> i32 {
    let mut safe_count = 0;
    for report in reports {
        if report.len() > 1 {
            let found_unsafe = report_safe(&report);
            if found_unsafe {
                for i in 0..report.len() {
                    let mut dampened_report = report.clone();
                    dampened_report.remove(i);
                    if ! report_safe(&dampened_report) {
                        safe_count += 1;
                        break;
                    }
                }
            } else {
                safe_count += 1;
            }
        }
    }
    safe_count
}

fn report_safe(report: &Vec<i32>) -> bool {
    let mut found_unsafe = false;
    let signum = (report[1] - report[0]).signum();
    for i in 1..report.len() {
        let delta = report[i] - report[i-1];
        if delta.signum() != signum || delta.abs() > 3 {
            found_unsafe = true;
            break;
        }
    }
    found_unsafe
}

fn count_occurrences(v: Vec<i32>) -> HashMap<i32, i32> {
    let mut c = HashMap::new();

    for i in v {
        c.entry(i).and_modify(|n| *n += 1).or_insert(1);
    }

    c
}
