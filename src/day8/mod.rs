// https://adventofcode.com/2022/day/8

use crate::get_input;
use std::cmp::max;
use std::i32;

const DAY: u8 = 8;
const MAX_SIZE: usize = 100;

pub fn main() {
    println!("{}", part1(&get_input(DAY, true)));
    println!("{}", part1(&get_input(DAY, false)));
    println!("{}", part2(&get_input(DAY, true)));
    println!("{}", part2(&get_input(DAY, false)));
}

fn part1(input: &str) -> i32 {
    let size = input.lines().next().unwrap().len();

    let mut visible_trees = [[false; MAX_SIZE]; MAX_SIZE];
    let mut tallest_top = [' '; MAX_SIZE];
    let mut tallest_left = [' '; MAX_SIZE];

    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char > tallest_top[j] {
                tallest_top[j] = char;
                visible_trees[i][j] = true;
            }
            if char > tallest_left[i] {
                tallest_left[i] = char;
                visible_trees[i][j] = true;
            }
        }
    }

    let mut result = 0;
    let mut tallest_bottom = [' '; MAX_SIZE];
    let mut tallest_right = [' '; MAX_SIZE];

    for (i, line) in input.lines().rev().enumerate() {
        for (j, char) in line.chars().rev().enumerate() {
            if char > tallest_bottom[j] {
                tallest_bottom[j] = char;
                visible_trees[size - i - 1][size - j - 1] = true;
            }
            if char > tallest_right[i] {
                tallest_right[i] = char;
                visible_trees[size - i - 1][size - j - 1] = true;
            }

            if visible_trees[size - i - 1][size - j - 1] {
                result += 1;
            }
        }
    }

    result
}

fn part2(input: &str) -> i32 {
    let size = input.lines().next().unwrap().len();

    let mut trees = [[' '; MAX_SIZE]; MAX_SIZE];

    let mut visible_left = [[0; MAX_SIZE]; MAX_SIZE];
    let mut visible_top = [[0; MAX_SIZE]; MAX_SIZE];
    let mut visible_right = [[0; MAX_SIZE]; MAX_SIZE];
    let mut visible_bottom = [[0; MAX_SIZE]; MAX_SIZE];

    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            trees[i][j] = char;
            for (number_of_trees, k) in (0..j).rev().enumerate() {
                if trees[i][k] >= trees[i][j] {
                    visible_left[i][j] = number_of_trees + 1;
                    break;
                }
            }
            if visible_left[i][j] == 0 {
                visible_left[i][j] = j;
            }
        }
    }

    for i in 0..size {
        for j in 0..size {
            for (number_of_trees, k) in (0..j).rev().enumerate() {
                if trees[k][i] >= trees[j][i] {
                    visible_top[j][i] = number_of_trees + 1;
                    break;
                }
            }
            if visible_top[j][i] == 0 {
                visible_top[j][i] = j;
            }
        }
    }

    for i in 0..size {
        for j in 0..size {
            for (number_of_trees, k) in (j + 1..size).enumerate() {
                if trees[i][k] >= trees[i][j] {
                    visible_right[i][j] = number_of_trees + 1;
                    break;
                }
            }
            if visible_right[i][j] == 0 {
                visible_right[i][j] = size - j - 1;
            }
        }
    }

    for i in 0..size {
        for j in 0..size {
            for (number_of_trees, k) in (j + 1..size).enumerate() {
                if trees[k][i] >= trees[j][i] {
                    visible_bottom[j][i] = number_of_trees + 1;
                    break;
                }
            }
            if visible_bottom[j][i] == 0 {
                visible_bottom[j][i] = size - j - 1;
            }
        }
    }

    let mut best = 0;

    for i in 0..size {
        for j in 0..size {
            let result =
                visible_left[i][j] * visible_right[i][j] * visible_top[i][j] * visible_bottom[i][j];
            best = max(best, result);
        }
    }

    best as i32
}

#[test]
fn test1() {
    let result = part1(&get_input(DAY, true));
    assert_eq!(result, 21);

    let result = part1(&get_input(DAY, false));
    assert_eq!(result, 1733);
}

#[test]
fn test2() {
    let result = part2(&get_input(DAY, true));
    assert_eq!(result, 8);

    let result = part2(&get_input(DAY, false));
    assert_eq!(result, 284648);
}
