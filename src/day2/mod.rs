// https://adventofcode.com/2022/day/2

use crate::get_input;

const DAY: u8 = 2;

pub fn main() {
    println!("{}", part1(&get_input(DAY, true)));
    println!("{}", part1(&get_input(DAY, false)));
    println!("{}", part2(&get_input(DAY, true)));
    println!("{}", part2(&get_input(DAY, false)));
}

fn process(input: &str, map: [[i32; 3]; 3], fun: &dyn Fn(&str, [[i32; 3]; 3]) -> i32) -> i32 {
    let mut score = 0;
    for line in input.lines() {
        let result = fun(line, map);
        score += result;
    }
    score
}

fn result1(line: &str, map: [[i32; 3]; 3]) -> i32 {
    let opponent = line[0..1].chars().next().unwrap();
    let you = line[2..3].chars().next().unwrap();

    let match_score = map[opponent as usize - 'A' as usize][you as usize - 'X' as usize];
    let figure_score = you as i32 - 'X' as i32 + 1;

    match_score + figure_score
}

fn part1(input: &str) -> i32 {
    // The first dimension is the opponent's figure - 'A'.
    // The second dimension is your figure - 'X';
    // The third dimension is the score of the match.
    let map = [[3, 6, 0], [0, 3, 6], [6, 0, 3]];

    process(input, map, &result1)
}

fn result2(line: &str, map: [[i32; 3]; 3]) -> i32 {
    let opponent = line[0..1].chars().next().unwrap();
    let expected_result = line[2..3].chars().next().unwrap();

    let figure_score =
        map[opponent as usize - 'A' as usize][expected_result as usize - 'X' as usize];
    let match_score = (expected_result as i32 - 'X' as i32) * 3;

    figure_score + match_score
}

fn part2(input: &str) -> i32 {
    // The first dimension is the opponent's figure - 'A'.
    // The second dimension is the expected result (0 - lost, 1 - draw, 2 - won).
    // The values are scores of the figures you should use.
    let map = [[3, 1, 2], [1, 2, 3], [2, 3, 1]];

    process(input, map, &result2)
}

#[test]
fn test1() {
    let result = part1(&get_input(DAY, true));
    assert_eq!(result, 15);

    let result2 = part1(&get_input(DAY, false));
    assert_eq!(result2, 13675);
}

#[test]
fn test2() {
    let result = part2(&get_input(DAY, true));
    assert_eq!(result, 12);

    let result2 = part2(&get_input(DAY, false));
    assert_eq!(result2, 14184);
}
