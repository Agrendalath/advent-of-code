// https://adventofcode.com/2022/day/1

use crate::get_input;
use std::cmp;
use std::str::FromStr;

const DAY: u8 = 1;

pub fn main() {
    println!("{}", part1(&get_input(DAY, true)));
    println!("{}", part1(&get_input(DAY, false)));
    println!("{}", part2(&get_input(DAY, true)));
    println!("{}", part2(&get_input(DAY, false)));
}

fn part1(input: &str) -> i32 {
    let mut current = 0;
    let mut max = 0;
    for line in input.lines() {
        if !line.is_empty() {
            current += i32::from_str(line).unwrap_or(0);
        } else {
            max = cmp::max(current, max);
            current = 0;
        }
    }
    cmp::max(max, current)
}

fn part2(input: &str) -> i32 {
    let mut vector = vec![0];
    for line in input.lines() {
        if !line.is_empty() {
            *vector.last_mut().unwrap() += i32::from_str(line).unwrap_or(0);
        } else {
            vector.push(0)
        }
    }
    vector.sort_by(|a, b| b.cmp(a));
    vector[0] + vector[1] + vector[2]
}

#[test]
fn test1() {
    let result = part1(&get_input(DAY, true));
    assert_eq!(result, 24000);

    let result = part1(&get_input(DAY, false));
    assert_eq!(result, 71023);
}

#[test]
fn test2() {
    let result = part2(&get_input(DAY, true));
    assert_eq!(result, 45000);

    let result = part2(&get_input(DAY, false));
    assert_eq!(result, 206289);
}
