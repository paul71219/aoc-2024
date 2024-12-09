
use std::str::FromStr;

fn s2cv(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn str_i32(s: &&str) -> Option<i32> {
    match i32::from_str(s) {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}
