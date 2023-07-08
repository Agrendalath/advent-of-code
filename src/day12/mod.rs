// https://adventofcode.com/2022/day/12

use crate::get_input;
use std::collections::{HashSet, VecDeque};

const DAY: u8 = 12;
const MAX_NODES: usize = 64;

pub fn main() {
    println!("{}", part1(&get_input(DAY, true)));
    println!("{}", part1(&get_input(DAY, false)));
    println!("{}", part2(&get_input(DAY, true)));
    println!("{}", part2(&get_input(DAY, false)));
}

fn update_neighbour_distance(
    current_node: (usize, usize),
    neighbour_node: (usize, usize),
    nodes: &[Vec<char>],
) -> bool {
    let neighbour_node = (neighbour_node.0, neighbour_node.1);
    let mut current_height = nodes[current_node.0][current_node.1] as u32;
    let mut neighbour_height = nodes[neighbour_node.0][neighbour_node.1] as u32;

    if current_height == 'S' as u32 {
        current_height = 'a' as u32;
    } else if current_height == 'E' as u32 {
        current_height = 'z' as u32;
    }
    if neighbour_height == 'S' as u32 {
        neighbour_height = 'a' as u32;
    } else if neighbour_height == 'E' as u32 {
        neighbour_height = 'z' as u32;
    }

    if current_height + 1 >= neighbour_height {
        return true;
    }

    false
}

fn dijkstra(nodes: &Vec<Vec<char>>, start_nodes: &[(usize, usize)]) -> i32 {
    let mut path = VecDeque::new();
    let mut visited = HashSet::new();
    let mut distance = vec![vec![i32::MAX; nodes[0].len()]; nodes.len()];

    for node in start_nodes {
        path.push_back(*node);
        distance[node.0][node.1] = 0;
    }

    let mut final_node = path[0];

    while !path.is_empty() {
        let current_node = path.pop_front().unwrap();
        if visited.contains(&current_node) {
            continue;
        }
        if nodes[current_node.0][current_node.1] == 'E' {
            final_node = current_node;
        }
        visited.insert(current_node);

        for relative_position in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            if (relative_position.0 == -1 && current_node.0 == 0)
                || (relative_position.1 == -1 && current_node.1 == 0)
                || (relative_position.0 == 1 && current_node.0 == nodes.len() - 1)
                || (relative_position.1 == 1 && current_node.1 == nodes[0].len() - 1)
            {
                continue;
            }
            let neighbour_node = (
                (current_node.0 as i32 + relative_position.0) as usize,
                (current_node.1 as i32 + relative_position.1) as usize,
            );
            if visited.contains(&neighbour_node) {
                continue;
            }

            if update_neighbour_distance(current_node, neighbour_node, nodes) {
                let latest_neighbour_distance = distance[neighbour_node.0][neighbour_node.1];
                let current_distance = distance[current_node.0][current_node.1] + 1;
                if current_distance < latest_neighbour_distance {
                    distance[neighbour_node.0][neighbour_node.1] = current_distance;
                    path.push_back(neighbour_node);

                    if start_nodes.len() > 1 {
                        visited.remove(&neighbour_node);
                    }
                }
            }
        }
    }

    distance[final_node.0][final_node.1]
}

fn solve(input: &str, multiple_starting_nodes: bool) -> i32 {
    let mut nodes: Vec<Vec<char>> = Vec::with_capacity(MAX_NODES);
    let mut start_nodes = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let mut nodes_line = Vec::with_capacity(MAX_NODES);
        for (j, char) in line.chars().enumerate() {
            nodes_line.push(char);
            if char == 'S' {
                start_nodes.push((i, j));
            }
            if multiple_starting_nodes && char == 'a' {
                start_nodes.push((i, j));
            }
        }
        nodes.push(nodes_line);
    }

    dijkstra(&nodes, &start_nodes)
}

fn part1(input: &str) -> i32 {
    solve(input, false)
}

fn part2(input: &str) -> i32 {
    solve(input, true)
}

#[test]
fn test() {
    let result = part1(&get_input(DAY, true));
    assert_eq!(result, 31);

    let result = part1(&get_input(DAY, false));
    assert_eq!(result, 370);

    let result = part2(&get_input(DAY, true));
    assert_eq!(result, 29);

    let result = part2(&get_input(DAY, false));
    assert_eq!(result, 363);
}
