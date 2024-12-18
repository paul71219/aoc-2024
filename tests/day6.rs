use std::{char, collections::HashSet};



const REAL_IN: &str = include_str!("../input/day6.txt");
const TEST_IN: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

pub fn score_1(room: &Vec<Vec<char>>) -> i32 {
    let mut room_marked = room.clone();
    patrol_room(&mut room_marked).unwrap()
}

pub fn score_2(room: &Vec<Vec<char>>) -> i32 {
    let mut room_marked = room.clone();
    patrol_room(&mut room_marked);

    // println!("{:?}", room);

    let mut count = 0;
    for x in 0..room.len() {
        let row = &room_marked[x];
        for y in 0..row.len() {
            if row[y] != 'X' || room[x][y] != '.' {
                continue;
            }
            let mut room_plus_object = room.clone();
            room_plus_object[x][y] = '#';
            if patrol_room(&mut room_plus_object) == None {
                //println!("Guard looped with object at {},{}", x, y);
                count += 1;
            }
        }
    }

    count
}

fn patrol_room(room: &mut Vec<Vec<char>>) -> Option<i32> {
    let mut guard_history = HashSet::new();
    let mut guard = find_guard(room);
    let mut count = 0;
    // println!("found guard at {:?}", guard);
    while guard.0 != '!' {
        if guard_history.contains(&guard) {
            return None;
        }
        guard_history.insert(guard);
        let (_, x, y) = guard;
        if room[x][y] != 'X' {
            room[x][y] = 'X';
            count += 1;
        }
        guard = patrol_step(guard, &room);
    }
    Some(count)
}

fn patrol_step(guard: (char, usize, usize), room_marked: &Vec<Vec<char>>) -> (char, usize, usize) {
    let (guard_dir, x_cur, y_cur) = guard;
    let (x_next, y_next, guard_next) = match guard {
        ('^', x, y) => (sub1(x), y, '>'),
        ('>', x, y) => (x, y+1, 'v'),
        ('v', x, y) => (x+1, y, '<'),
        ('<', x, y) => (x, sub1(y), '^'),
        g => panic!("unexpected guard state {:?}", g),
    };

    match room_marked.get(x_next).and_then(|row| row.get(y_next)) {
        Some('#') => (guard_next, x_cur, y_cur),
        Some('.') => (guard_dir, x_next, y_next),
        Some('X') => (guard_dir, x_next, y_next),
        None => ('!', 0, 0),
        Some(c) => panic!("Unexpected location character {} at {},{}", c, x_next, y_next),
    }
}

fn sub1(i: usize) -> usize {
    i.overflowing_sub(1).0
}

fn find_guard(room: &Vec<Vec<char>>) -> (char, usize, usize) {
    for x in 0..room.len()-1 {
        let row = &room[x];
        for y in 0..row.len()-1 {
            if "<>^v".contains(row[y]) {
                return (row[y], x, y);
            }
        }
    }

    panic!("Could not find guard in room\n{:?}", room);
}


fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.split('\n').filter(|x| x.len() > 1).map(s2cv).collect()
}

fn s2cv(line: &str) -> Vec<char> {
    line.chars().collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        let sample_parsed = parse_input(TEST_IN);
        let real_parsed = parse_input(REAL_IN);
        let sample_1 = score_1(&sample_parsed);
        println!("Sample 1: {}", sample_1);
        assert_eq!(sample_1, 41);
        let real_1 = score_1(&real_parsed);
        println!("Part 1: {}", real_1);

        let sample_2 = score_2(&sample_parsed);
        println!("Sample 2: {}", sample_2);
        assert_eq!(sample_2, 6);
        let real_2 = score_2(&real_parsed);
        println!("Part 2: {}", real_2);
    }
}