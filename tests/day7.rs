use std::char;
use std::str::FromStr;


const REAL_IN: &str = include_str!("../input/day7.txt");
const TEST_IN: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

struct EquationNums {
    goal: i64,
    numbers: Vec<i32>,
}

fn parse_input(input: &str) -> Vec<EquationNums> {
    input.split('\n').filter(|x| x.len() > 1).filter_map(parse_line).collect()
}

fn parse_line(line: &str) -> Option<EquationNums> {
    if line.len() == 0 {
        return None;
    }
    let s1: Vec<&str> = line.split(": ").collect();
    if s1.len() != 2 {
        panic!("Unexpected input '{:?}'", line)
    }
    let s2 = s1[1].split_whitespace().map(|s| i32::from_str(s).expect("bad number")).collect();
    Some(EquationNums { goal: i64::from_str(s1[0]).expect("bad number"), numbers: s2 })
}


fn str_i32(s: &&str) -> Option<i32> {
    match i32::from_str(s) {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}

fn score_part_1(goal: i64, so_far: i64, remaining: &[i32]) -> bool {
    if remaining.len() == 0 {
        return so_far == goal;
    }
    if (so_far > goal) {
        return false;
    }
    return score_part_1(goal, so_far + (remaining[0] as i64), &remaining[1..]) 
    || score_part_1(goal, so_far * (remaining[0] as i64), &remaining[1..]) 

}

fn score_line_1(eqn: &EquationNums) -> Option<i64> {
    match score_part_1(eqn.goal, eqn.numbers[0] as i64, &eqn.numbers[1..]) {
        true => Some(eqn.goal),
        false => None,
    }
}

fn score_1(eqs: &[EquationNums]) -> i64 {
    eqs.iter().filter_map(&score_line_1).reduce(|a, b| a + b).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        println!("source {}", file!());

        let sample_parsed = parse_input(TEST_IN);
        let real_parsed = parse_input(REAL_IN);
        let sample_1 = score_1(&sample_parsed);
        println!("Sample 1: {}", sample_1);
        assert_eq!(sample_1, 3749);
        let real_1 = score_1(&real_parsed);
        println!("Part 1: {}", real_1);

        let sample_2 = score_2(&sample_parsed);
        println!("Sample 2: {}", sample_2);
        assert_eq!(sample_2, 11387);
        let real_2 = score_2(&real_parsed);
        println!("Part 2: {}", real_2);
    }
}