// https://adventofcode.com/2022/day/3

use crate::get_input;
use std::collections::HashSet;

const DAY: u8 = 3;

pub fn main() {
    println!("{}", part1(&get_input(DAY, true)));
    println!("{}", part1(&get_input(DAY, false)));
    println!("{}", part2(&get_input(DAY, true)));
    println!("{}", part2(&get_input(DAY, false)));
}

fn get_priority(item_type: char) -> i32 {
    if item_type <= 'Z' {
        return item_type as i32 - 'A' as i32 + 27;
    }

    item_type as i32 - 'a' as i32 + 1
}

fn result1(line: &str) -> i32 {
    let size = line.len() / 2;
    let first = &line[..size];
    let second = &line[size..];

    let first: HashSet<char> = first.chars().collect();
    let second: HashSet<char> = second.chars().collect();

    let common_char = first.intersection(&second).last().copied().unwrap();

    get_priority(common_char)
}

fn part1(input: &str) -> i32 {
    let mut score = 0;
    for line in input.lines() {
        let result = result1(line);
        score += result;
    }
    score
}

fn result2(first: &str, second: &str, third: &str) -> i32 {
    // println!("{}, {}, {}", first, second, third);

    let first: HashSet<char> = first.chars().collect();
    let second: HashSet<char> = second.chars().collect();
    let third: HashSet<char> = third.chars().collect();

    let intersection: HashSet<char> = first.intersection(&second).copied().collect();
    let common_char = intersection.intersection(&third).last().copied().unwrap();
    // println!("{}", common_char);

    get_priority(common_char)
}

fn part2(input: &str) -> i32 {
    let mut score = 0;
    let mut rucksacks: [&str; 3] = ["empty"; 3];

    for (i, line) in input.lines().enumerate() {
        rucksacks[i % 3] = line;

        if i % 3 == 2 {
            score += result2(rucksacks[0], rucksacks[1], rucksacks[2]);
        }
    }
    score
}

#[test]
fn test1() {
    let result = part1(&get_input(DAY, true));
    assert_eq!(result, 157);

    let result = part1(&get_input(DAY, false));
    assert_eq!(result, 7889);
}

#[test]
fn test2() {
    let result = part2(&get_input(DAY, true));
    assert_eq!(result, 70);

    let result = part2(&get_input(DAY, false));
    assert_eq!(result, 2825);
}
