// https://adventofcode.com/2022/day/6

use crate::get_input;
use std::collections::VecDeque;

const DAY: u8 = 6;

const TEST_INPUTS: [(&str, i32, i32); 5] = [
    ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
    ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
    ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
    ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
    ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
];

pub fn main() {
    for input in TEST_INPUTS {
        println!("{}", part1(input.0));
    }
    println!("{}", part1(&get_input(DAY, false)));
    for input in TEST_INPUTS {
        println!("{}", part1(input.0));
    }
    println!("{}", part2(&get_input(DAY, false)));
}

fn find_unique_index(input: &str, unique_chars: usize) -> i32 {
    let mut sequence: VecDeque<char> = VecDeque::new();
    let mut index = 0;

    for (i, letter) in input.chars().enumerate() {
        while sequence.contains(&letter) {
            sequence.pop_front();
        }
        sequence.push_back(letter);

        if sequence.len() == unique_chars {
            index = i as i32 + 1;
            break;
        }
    }
    index
}

fn part1(input: &str) -> i32 {
    find_unique_index(input, 4)
}

fn part2(input: &str) -> i32 {
    find_unique_index(input, 14)
}

#[test]
fn test1() {
    for input in TEST_INPUTS {
        let result = part1(input.0);
        assert_eq!(result, input.1);
    }

    let result = part1(&get_input(DAY, false));
    assert_eq!(result, 1582);
}

#[test]
fn test2() {
    for input in TEST_INPUTS {
        let result = part2(input.0);
        assert_eq!(result, input.2);
    }

    let result = part2(&get_input(DAY, false));
    assert_eq!(result, 3588);
}
