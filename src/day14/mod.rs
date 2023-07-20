// https://adventofcode.com/2022/day/14

use crate::get_input;
use std::cmp::{max, min};
use std::str::Split;

const DAY: u8 = 14;
const X_MAX: usize = 1000;
const X_SHIFT: usize = 100;
const Y_MAX: usize = 168;
const SAND_SOURCE: usize = 500 + X_SHIFT;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Boundaries {
    lower: Point,
    upper: Point,
}

pub fn main() {
    println!("{}", part1(&get_input(DAY, true)));
    println!("{}", part1(&get_input(DAY, false)));
    println!("{}", part2(&get_input(DAY, true)));
    println!("{}", part2(&get_input(DAY, false)));
}

#[allow(dead_code)]
fn show_cave(cave: &[Vec<char>], boundaries: &Boundaries) {
    for y in 0..=boundaries.upper.y {
        for x in boundaries.lower.x..=boundaries.upper.x {
            print!("{}", cave[x][y]);
        }
        println!();
    }
    println!();
}

fn place_sand(cave: &mut [Vec<char>], boundaries: &Boundaries, stop_at_the_top: bool) -> u32 {
    let mut result = 0;

    'main: loop {
        let mut x = SAND_SOURCE;
        if stop_at_the_top && cave[x][0] == 'o' {
            // show_cave(cave, boundaries);
            return result;
        }

        for y in 0..=boundaries.upper.y {
            if cave[x][y + 1] == '.' {
                continue;
            } else {
                if cave[x - 1][y + 1] == '.' {
                    if x == boundaries.lower.x {
                        // show_cave(cave, boundaries);
                        return result;
                    }

                    x -= 1;
                    continue;
                }

                if cave[x + 1][y + 1] == '.' {
                    if x == boundaries.upper.x {
                        // show_cave(cave, boundaries);
                        return result;
                    }

                    x += 1;
                    continue;
                }
            }

            cave[x][y] = 'o';
            result += 1;
            continue 'main;
        }

        // show_cave(cave, boundaries);
        return result;
    }
}

fn create_path(split: Split<&str>, cave: &mut [Vec<char>], boundaries: &mut Boundaries) {
    let mut previous_coordinates = Point { x: X_MAX, y: X_MAX };
    for coordinates in split {
        let mut split_coordinates = coordinates.split(',').take(2);
        let x: usize = split_coordinates.next().unwrap().parse().unwrap();
        let x = x + X_SHIFT;
        let y: usize = split_coordinates.next().unwrap().parse().unwrap();

        // Ignore this for the first run.
        if previous_coordinates.x == X_MAX {
            previous_coordinates.x = x;
            previous_coordinates.y = y;
        }

        let range_x = min(x, previous_coordinates.x)..=max(x, previous_coordinates.x);
        let range_y = min(y, previous_coordinates.y)..=max(y, previous_coordinates.y);
        for i in range_x {
            cave[i][y] = '#';
        }
        for i in range_y {
            cave[x][i] = '#';
        }
        previous_coordinates.x = x;
        previous_coordinates.y = y;

        boundaries.lower.x = min(x, boundaries.lower.x);
        boundaries.upper.x = max(x, boundaries.upper.x);
        boundaries.lower.y = min(y, boundaries.lower.y);
        boundaries.upper.y = max(y, boundaries.upper.y);
    }
}

fn part1(input: &str) -> u32 {
    let mut cave = vec![vec!['.'; Y_MAX]; X_MAX];
    let mut boundaries = Boundaries {
        lower: Point { x: X_MAX, y: X_MAX },
        upper: Point { x: 0, y: 0 },
    };

    for line in input.lines() {
        create_path(line.split(" -> "), &mut cave, &mut boundaries);
    }

    place_sand(&mut cave, &boundaries, false)
}

fn part2(input: &str) -> u32 {
    let mut cave = vec![vec!['.'; Y_MAX + 2]; X_MAX];
    let mut boundaries = Boundaries {
        lower: Point {
            x: X_MAX,
            y: Y_MAX + 2,
        },
        upper: Point { x: 0, y: 0 },
    };
    for line in input.lines() {
        create_path(line.split(" -> "), &mut cave, &mut boundaries);
    }

    boundaries.lower.x -= 200;
    boundaries.upper.x += 200;
    boundaries.upper.y += 2;

    for x in 0..X_MAX - 1 {
        cave[x][boundaries.upper.y] = '#'
    }

    place_sand(&mut cave, &boundaries, true)
}

#[test]
fn test1() {
    let result = part1(&get_input(DAY, true));
    assert_eq!(result, 24);

    let result = part1(&get_input(DAY, false));
    assert_eq!(result, 768);
}

#[test]
fn test2() {
    let result = part2(&get_input(DAY, true));
    assert_eq!(result, 93);

    let result = part2(&get_input(DAY, false));
    assert_eq!(result, 26686);
}
