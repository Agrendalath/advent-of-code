// https://adventofcode.com/2022/day/7

use crate::get_input;
use std::i32;
use std::str::FromStr;

const DAY: u8 = 7;

pub fn main() {
    println!("{}", part1(&get_input(DAY, true)));
    println!("{}", part1(&get_input(DAY, false)));
    println!("{}", part2(&get_input(DAY, true)));
    println!("{}", part2(&get_input(DAY, false)));
}

fn get_files_size(buffer: &Vec<&str>) -> i32 {
    let mut size = 0;
    for line in buffer {
        let mut split = line.split(' ');
        let dir_or_size = split.next().unwrap();

        size += i32::from_str(dir_or_size).unwrap_or(0)
    }
    size
}

fn part1(input: &str) -> i32 {
    const MAX_SIZE: i32 = 100000;

    let mut result = 0;
    let mut buffer: Vec<&str> = vec![];
    let mut sizes: Vec<i32> = vec![];

    for line in input.lines() {
        if line.starts_with('$') {
            if !buffer.is_empty() {
                sizes.push(get_files_size(&buffer));
                buffer.clear();
            }
            if line.starts_with("$ cd") {
                let file_name = line[5..].to_string();
                if file_name == ".." {
                    let current_size = sizes.pop().unwrap();
                    if current_size <= MAX_SIZE {
                        result += current_size;
                    }
                    let previous_size = sizes.pop().unwrap() + current_size;
                    sizes.push(previous_size);
                }
            }
        } else {
            buffer.push(line);
        }
    }

    if !buffer.is_empty() {
        sizes.push(get_files_size(&buffer));
        buffer.clear();
    }
    while !sizes.is_empty() {
        let current_size = sizes.pop().unwrap();
        if current_size <= MAX_SIZE {
            result += current_size;
        }

        if !sizes.is_empty() {
            let previous_size = sizes.pop().unwrap() + current_size;
            sizes.push(previous_size);
        }
    }

    result
}

fn part2(input: &str) -> i32 {
    const TOTAL_SPACE: i32 = 70000000;
    const REQUIRED_SPACE: i32 = 30000000;

    let mut buffer: Vec<&str> = vec![];
    let mut sizes: Vec<i32> = vec![];
    let mut all_sizes: Vec<i32> = vec![];

    for line in input.lines() {
        if line.starts_with('$') {
            if !buffer.is_empty() {
                sizes.push(get_files_size(&buffer));
                buffer.clear();
            }
            if line.starts_with("$ cd") {
                let file_name = line[5..].to_string();
                if file_name == ".." {
                    let current_size = sizes.pop().unwrap();
                    all_sizes.push(current_size);

                    let previous_size = sizes.pop().unwrap() + current_size;
                    sizes.push(previous_size);
                }
            }
        } else {
            buffer.push(line);
        }
    }

    if !buffer.is_empty() {
        sizes.push(get_files_size(&buffer));
        buffer.clear();
    }
    while !sizes.is_empty() {
        let current_size = sizes.pop().unwrap();
        all_sizes.push(current_size);

        if !sizes.is_empty() {
            let previous_size = sizes.pop().unwrap() + current_size;
            sizes.push(previous_size);
        }
    }

    all_sizes.sort();
    let free_space = TOTAL_SPACE - all_sizes.last().unwrap();
    for size in all_sizes {
        if free_space + size >= REQUIRED_SPACE {
            return size;
        }
    }

    -1
}

#[test]
fn test() {
    let result = part1(&get_input(DAY, true));
    assert_eq!(result, 95437);

    let result = part1(&get_input(DAY, false));
    assert_eq!(result, 1513699);

    let result = part2(&get_input(DAY, true));
    assert_eq!(result, 24933642);

    let result = part2(&get_input(DAY, false));
    assert_eq!(result, 7991939);
}
