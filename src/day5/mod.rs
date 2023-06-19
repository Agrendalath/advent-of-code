// https://adventofcode.com/2022/day/5

use crate::get_input;
use regex::Regex;
use std::str::FromStr;
use std::usize;
// use std::collections::HashSet;

const DAY: u8 = 5;

pub fn main() {
    println!("{}", part1(&get_input(DAY, true)));
    println!("{}", part1(&get_input(DAY, false)));
    println!("{}", part2(&get_input(DAY, true)));
    println!("{}", part2(&get_input(DAY, false)));
}

// fn process(input: &str, function: &dyn Fn(&str) -> i32) -> i32 {
//     let mut score = 0;
//     for line in input.lines() {
//         let result = function(line);
//         score += result;
//     }
//     score
// }
//
// fn result1(line: &str) -> i32 {
//     let split: Vec<&str> = line.split(',').collect();
//     let first_split: Vec<&str> = split[0].split('-').collect();
//     let second_split: Vec<&str> = split[1].split('-').collect();
//
//     let first_lower = i32::from_str(first_split[0]).unwrap();
//     let first_upper = i32::from_str(first_split[1]).unwrap();
//
//     let second_lower = i32::from_str(second_split[0]).unwrap();
//     let second_upper = i32::from_str(second_split[1]).unwrap();
//
//     if (first_lower <= second_lower && first_upper >= second_upper)
//         || (second_lower <= first_lower && second_upper >= first_upper)
//     {
//         return 1;
//     }
//     0
// }

fn load_stack_line(line: &str, stacks: &mut [Vec<char>]) {
    let mut index = 0;
    loop {
        let current_crate = line[index + 1..=index + 1].chars().next().unwrap();

        // dbg!(current_crate);
        if !current_crate.is_whitespace() && !current_crate.is_numeric() {
            stacks[index / 4].push(current_crate);
        }

        index += 4;
        if index >= line.len() - 1 {
            // dbg!(index, line.len());
            break;
        }
    }
}

fn reverse_stacks(stacks: &mut [Vec<char>]) {
    for stack in stacks {
        stack.reverse();
    }
}

fn process_with_single_move(stacks: &mut Vec<Vec<char>>, instructions: Vec<&str>) {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for instruction in instructions {
        let captures = re.captures_iter(instruction).last().unwrap();
        let number_of_crates = i32::from_str(&captures[1]).unwrap();
        let from = usize::from_str(&captures[2]).unwrap() - 1;
        let to = usize::from_str(&captures[3]).unwrap() - 1;

        for _ in 0..number_of_crates {
            let current_crate = stacks[from].pop().unwrap();
            stacks[to].push(current_crate);
        }
    }
}

fn process_with_multiple_moves(stacks: &mut Vec<Vec<char>>, instructions: Vec<&str>) {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for instruction in instructions {
        let captures = re.captures_iter(instruction).last().unwrap();
        let number_of_crates = usize::from_str(&captures[1]).unwrap();
        let from = usize::from_str(&captures[2]).unwrap() - 1;
        let to = usize::from_str(&captures[3]).unwrap() - 1;

        // This can probably be done much more efficient, but I don't know how to use vector slices.
        let mut temporary_stack: Vec<char> = vec![];
        for _ in 0..number_of_crates {
            temporary_stack.push(stacks[from].pop().unwrap());
        }
        temporary_stack.reverse();
        for current_crate in temporary_stack {
            stacks[to].push(current_crate);
        }
    }
}

fn show_stack_tops(stacks: Vec<Vec<char>>) -> String {
    let mut result: Vec<String> = vec![];

    for stack in stacks {
        let current_crate = stack.last().unwrap_or(&' ');
        if !current_crate.is_whitespace() {
            result.push(current_crate.to_string());
        }
    }

    result.join("")
}

fn task(input: &str, processing_function: &dyn Fn(&mut Vec<Vec<char>>, Vec<&str>)) -> String {
    let mut stack_loaded = false;

    let mut stacks: Vec<Vec<char>> = vec![vec![]; 10];
    let mut instructions: Vec<&str> = vec![];

    for line in input.lines() {
        if line.is_empty() {
            stack_loaded = true;
        } else if stack_loaded {
            instructions.push(line);
        } else {
            load_stack_line(line, &mut stacks);
        }
    }

    reverse_stacks(&mut stacks);
    processing_function(&mut stacks, instructions);

    show_stack_tops(stacks)
}

fn part1(input: &str) -> String {
    task(input, &process_with_single_move)
}

fn part2(input: &str) -> String {
    task(input, &process_with_multiple_moves)
}

#[test]
fn test() {
    let result = part1(&get_input(DAY, true));
    assert_eq!(result, "CMZ");

    let result = part1(&get_input(DAY, false));
    assert_eq!(result, "NTWZZWHFV");

    let result = part2(&get_input(DAY, true));
    assert_eq!(result, "MCD");

    let result = part2(&get_input(DAY, false));
    assert_eq!(result, "BRZGFVBTJ");
}
