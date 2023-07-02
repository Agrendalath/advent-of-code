// https://adventofcode.com/2022/day/9

use std::collections::HashSet;
use std::i32;
use std::str::FromStr;

use crate::get_input;

const DAY: u8 = 9;
const MAX_NODES: usize = 10;

pub fn main() {
    println!("{}", part1(&get_input(DAY, true)));
    println!("{}", part1(&get_input(DAY, false)));
    println!("{}", part2(&get_input(DAY, true)));
    println!("{}", part2(&get_input(DAY, false)));
}

fn move_node(direction: &str, node: &mut [i32; 2]) {
    match direction {
        "U" => {
            node[1] += 1;
        }
        "D" => {
            node[1] -= 1;
        }
        "L" => {
            node[0] -= 1;
        }
        "R" => {
            node[0] += 1;
        }
        _ => panic!("Unknown command."),
    }
}

fn normalize_node(parent_node: [i32; 2], node: &mut [i32; 2]) {
    let x_distance = parent_node[0] - node[0];
    let y_distance = parent_node[1] - node[1];
    // Special case - diagonal move.
    if x_distance.abs() > 1 && y_distance.abs() > 1 {
        node[0] += x_distance.signum();
        node[1] += y_distance.signum();
    } else if x_distance.abs() > 1 {
        node[0] += x_distance.signum();
        node[1] += parent_node[1] - node[1];
    } else if y_distance.abs() > 1 {
        node[1] += y_distance.signum();
        node[0] += parent_node[0] - node[0];
    }
}

fn solve(input: &str, nodes_number: usize) -> usize {
    let mut visited: HashSet<[i32; 2]> = HashSet::new();
    let mut nodes: [[i32; 2]; MAX_NODES] = [[0, 0]; MAX_NODES];
    visited.insert([0, 0]);

    for line in input.lines() {
        let direction = &line[..1];
        let steps = i32::from_str(&line[2..]).unwrap();

        for _ in 0..steps {
            move_node(direction, &mut nodes[0]);
            for current_node in 1..nodes_number {
                normalize_node(nodes[current_node - 1], &mut nodes[current_node]);
            }
            visited.insert(nodes[nodes_number - 1]);
        }
    }

    visited.len()
}

fn part1(input: &str) -> usize {
    solve(input, 2)
}

fn part2(input: &str) -> usize {
    solve(input, 10)
}

#[test]
fn test() {
    let result = part1(&get_input(DAY, true));
    assert_eq!(result, 13);

    let result = part1(&get_input(DAY, false));
    assert_eq!(result, 6367);

    let result = part2(&get_input(DAY, true));
    assert_eq!(result, 1);

    let test_input_2 = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";

    let result = part2(test_input_2);
    assert_eq!(result, 36);

    let result = part2(&get_input(DAY, false));
    assert_eq!(result, 2536);
}
