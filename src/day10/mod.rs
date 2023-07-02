// https://adventofcode.com/2022/day/10

use std::i32;
use std::str::FromStr;

use crate::get_input;

const DAY: u8 = 10;

pub fn main() {
    println!("{}", part1(&get_input(DAY, true)));
    println!("{}", part1(&get_input(DAY, false)));
    part2(&get_input(DAY, true), true);
    part2(&get_input(DAY, false), true);
}

fn part1(input: &str) -> i32 {
    let mut result = 0;
    let mut register = 1;
    let mut cycle = 1;
    let mut next_cycle_check = 20;

    for line in input.lines() {
        let current_cycle_register = register;
        if line == "noop" {
            cycle += 1;
        } else {
            let value = i32::from_str(&line[5..]).unwrap();
            cycle += 2;
            register += value;
        }

        if cycle == next_cycle_check {
            result += cycle * register;
            next_cycle_check += 40;
        } else if cycle - 1 == next_cycle_check {
            result += (cycle - 1) * current_cycle_register;
            next_cycle_check += 40;
        }
    }

    result
}

fn should_draw(cycle: i32, register: i32) -> bool {
    let sprite_distance = cycle % 40 - register;
    (0..3).contains(&sprite_distance)
}

fn part2(input: &str, print_result: bool) -> [[bool; 40]; 6] {
    let mut register = 0_i32;
    let mut cycle = 0_usize;
    let mut screen = [[false; 40]; 6];

    for line in input.lines() {
        screen[cycle / 40][cycle % 40] = should_draw(cycle as i32, register);

        if line == "noop" {
            cycle += 1;
        } else {
            let value = i32::from_str(&line[5..]).unwrap();
            cycle += 1;
            screen[cycle / 40][cycle % 40] = should_draw(cycle as i32, register);
            cycle += 1;
            register += value;
        }
    }
    if cycle / 40 < 6 {
        screen[cycle / 40][cycle % 40] = should_draw(cycle as i32, register);
    }

    if print_result {
        for line in screen {
            for pixel in line {
                if pixel {
                    print!("#");
                } else {
                    print!(".")
                }
            }
            println!();
        }
    }

    screen
}

#[test]
fn test() {
    let result = part1(&get_input(DAY, true));
    assert_eq!(result, 13140);

    let result = part1(&get_input(DAY, false));
    assert_eq!(result, 13820);

    let test_result_2 = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
    let mut test_result_2_bool = [[false; 40]; 6];
    for (i, line) in test_result_2.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            test_result_2_bool[i][j] = char == '#';
        }
    }

    let result = part2(&get_input(DAY, true), false);
    assert_eq!(result, test_result_2_bool);

    println!();

    let test_result_2 = "####.#..#..##..###..#..#..##..###..#..#.
...#.#.#..#..#.#..#.#.#..#..#.#..#.#.#..
..#..##...#....#..#.##...#....#..#.##...
.#...#.#..#.##.###..#.#..#.##.###..#.#..
#....#.#..#..#.#.#..#.#..#..#.#.#..#.#..
####.#..#..###.#..#.#..#..###.#..#.#..#.";
    let mut test_result_2_bool = [[false; 40]; 6];
    for (i, line) in test_result_2.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            test_result_2_bool[i][j] = char == '#';
        }
    }

    let result = part2(&get_input(DAY, false), false);
    assert_eq!(result, test_result_2_bool);
}
