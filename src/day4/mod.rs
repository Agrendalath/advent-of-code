// https://adventofcode.com/2022/day/4

use crate::get_input;
use std::str::FromStr;
// use std::collections::HashSet;

const DAY: u8 = 4;

pub fn main() {
    println!("{}", part1(&get_input(DAY, true)));
    println!("{}", part1(&get_input(DAY, false)));
    println!("{}", part2(&get_input(DAY, true)));
    println!("{}", part2(&get_input(DAY, false)));
}

fn process(input: &str, function: &dyn Fn(&str) -> i32) -> i32 {
    let mut score = 0;
    for line in input.lines() {
        let result = function(line);
        score += result;
    }
    score
}

fn result1(line: &str) -> i32 {
    let split: Vec<&str> = line.split(',').collect();
    let first_split: Vec<&str> = split[0].split('-').collect();
    let second_split: Vec<&str> = split[1].split('-').collect();

    let first_lower = i32::from_str(first_split[0]).unwrap();
    let first_upper = i32::from_str(first_split[1]).unwrap();

    let second_lower = i32::from_str(second_split[0]).unwrap();
    let second_upper = i32::from_str(second_split[1]).unwrap();

    if (first_lower <= second_lower && first_upper >= second_upper)
        || (second_lower <= first_lower && second_upper >= first_upper)
    {
        return 1;
    }
    0
}

fn part1(input: &str) -> i32 {
    process(input, &result1)
}

fn result2(line: &str) -> i32 {
    let split: Vec<&str> = line.split(',').collect();
    let first_split: Vec<&str> = split[0].split('-').collect();
    let second_split: Vec<&str> = split[1].split('-').collect();

    let first_lower = i32::from_str(first_split[0]).unwrap();
    let first_upper = i32::from_str(first_split[1]).unwrap();

    let second_lower = i32::from_str(second_split[0]).unwrap();
    let second_upper = i32::from_str(second_split[1]).unwrap();

    let first_range = first_lower..=first_upper;
    let second_range = second_lower..=second_upper;

    if first_range.contains(&second_lower)
        || first_range.contains(&second_upper)
        || second_range.contains(&first_lower)
        || second_range.contains(&first_upper)
    {
        return 1;
    }
    0
}

fn part2(input: &str) -> i32 {
    process(input, &result2)
}

#[test]
fn test() {
    let result = part1(&get_input(DAY, true));
    assert_eq!(result, 2);

    let result = part1(&get_input(DAY, false));
    assert_eq!(result, 584);

    let result = part2(&get_input(DAY, true));
    assert_eq!(result, 4);

    let result = part2(&get_input(DAY, false));
    assert_eq!(result, 933);
}
