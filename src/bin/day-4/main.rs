use std::collections::HashMap;
use std::{fs, usize};
use std::io::Lines;
use std::str::FromStr;

fn main() {
    let in_filename = "/Users/pauld/w/aoc-2024/aoc_2024_pauldf/src/bin/day-4/sample.txt";

    let s = score_file(in_filename);
    if s != 9 {
        panic!("unexpected score {}", s);
    }
    println!("solved sample!");

    let in_filename = "/Users/pauld/w/aoc-2024/aoc_2024_pauldf/src/bin/day-4/input.txt";

    let s = score_file(in_filename);

    println!("{}", s);
}

fn score_file(filename: &str) -> i32 {
    let input = fs::read_to_string(filename).expect("failed to read fine");
    let lines = input.split('\n').filter(|x| x.len() > 1).map(s2cv).collect();
    score_b(lines)
}

fn s2cv(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn str_i32(s: &&str) -> Option<i32> {
    match i32::from_str(s) {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}

fn score_a(lines: Vec<Vec<char>>) -> i32 {

    let dirs: [(isize, isize); 8] = [(1,1), (1,0), (1,-1), (0,-1), (-1,-1), (-1, 0), (-1, 1), (0, 1)];
    let letters: [(isize, char); 3] = [(3, 'S'), (2, 'A'), (1, 'M')];

    let mut count = 0;
    for x in 0..(lines.len()-1) {
        let line = &lines[x];
        for y in 0..(line.len()-1) {
            if line[y] != 'X' {
                continue;
            }
            'for_dirs: for (dx, dy) in dirs {
                for (d, ch) in letters {
                    if ! ( lines.get((x as isize + dx*d) as usize)
                    .and_then(|l| l.get((y as isize +dy*d) as usize))
                    .and_then(|c| Some(*c)) == Some(ch) ) {
                        continue 'for_dirs;
                    }
                }

                count += 1;
                println!("found ({}, {}) d({}, {})", x, y, dx, dy);
            }
        }
    }
    count
}

fn score_b(lines: Vec<Vec<char>>) -> i32 {

    let mut count = 0;
    for x in 1..(lines.len()-1) {
        let line = &lines[x];
        for y in 1..(line.len()-1) {
            if line[y] != 'A' {
                continue;
            }
            match (lines[x-1][y-1], lines[x+1][y+1]) {
                ('M', 'S') => (),
                ('S', 'M') => (),
                _ => continue,
            }
            match (lines[x-1][y+1], lines[x+1][y-1]) {
                ('M', 'S') => (),
                ('S', 'M') => (),
                _ => continue,
            }
            count += 1;
        }
    }
    count
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
