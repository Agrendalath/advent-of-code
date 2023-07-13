// https://adventofcode.com/2022/day/11

use crate::get_input;
use regex::Regex;
use std::collections::VecDeque;
use std::ops::{Add, Div, Mul, Sub};

const DAY: u8 = 11;
const MAX_MONKEYS: usize = 8;

pub fn main() {
    println!("{}", part1(&get_input(DAY, true)));
    println!("{}", part1(&get_input(DAY, false)));
    println!("{}", part2(&get_input(DAY, true)));
    println!("{}", part2(&get_input(DAY, false)));
}

fn gcd(a: i32, b: i32) -> i32 {
    if b != 0 {
        return gcd(b, a % b);
    }
    a
}

fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i32>,
    operation_operator: char,
    operation_number: Option<i32>,
    inspection_divider: i32,
    divisible_by: i32,
    divisible_target: i32,
    indivisible_target: i32,
}

impl Monkey {
    fn add_item(&mut self, value: i32) {
        self.items.push_back(value);
    }

    fn throw(&mut self, modulo: i32) -> (i32, i32) {
        let value = self.items.pop_front().unwrap();
        let operation: fn(i64, i64) -> i64 = match self.operation_operator {
            '+' => i64::add,
            '-' => i64::sub,
            '*' => i64::mul,
            '/' => i64::div,
            _ => panic!("Unknown operator."),
        };

        let value = operation(value as i64, self.operation_number.unwrap_or(value) as i64)
            / self.inspection_divider as i64
            % modulo as i64;
        let value = value as i32;
        if value % self.divisible_by == 0 {
            return (value, self.divisible_target);
        }

        (value, self.indivisible_target)
    }
}

fn create_monkey(buffer: &[&str], inspection_divider: i32) -> Monkey {
    let items = buffer[1]
        .strip_prefix("  Starting items: ")
        .unwrap()
        .replace(' ', "");
    let items: Result<Vec<i32>, _> = items.split(',').map(|x| x.parse::<i32>()).collect();
    let items: VecDeque<i32> = VecDeque::from(items.unwrap());

    let operation_expression: &str = buffer[2].split("= ").last().unwrap();

    let re = Regex::new(r"[*+-/]").unwrap();
    let operation_operator = re.captures_iter(operation_expression).last().unwrap()[0]
        .chars()
        .last()
        .unwrap();

    let operation_number_str: Vec<&str> = operation_expression.matches(char::is_numeric).collect();
    let mut operation_number: Option<i32> = None;

    if !operation_number_str.is_empty() {
        operation_number = Some(operation_number_str.join("").parse().unwrap());
    }

    let divisible_by: Vec<&str> = buffer[3].matches(char::is_numeric).collect();
    let divisible_by: i32 = divisible_by.join("").parse().unwrap();

    let divisible_target: Vec<&str> = buffer[4].matches(char::is_numeric).collect();
    let divisible_target: i32 = divisible_target.join("").parse().unwrap();

    let indivisible_target: Vec<&str> = buffer[5].matches(char::is_numeric).collect();
    let indivisible_target: i32 = indivisible_target.join("").parse().unwrap();

    Monkey {
        items,
        operation_operator,
        operation_number,
        inspection_divider,
        divisible_by,
        divisible_target,
        indivisible_target,
    }
}

fn import_monkeys(input: &str, inspection_divider: i32) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];
    let mut buffer: Vec<&str> = vec![];

    for line in input.lines() {
        if line.is_empty() {
            monkeys.push(create_monkey(&buffer, inspection_divider));
            buffer.clear();
        } else {
            buffer.push(line);
        }
    }
    monkeys.push(create_monkey(&buffer, inspection_divider));

    monkeys
}

fn solve(input: &str, rounds: i32, inspection_divider: i32) -> i64 {
    let mut monkeys = import_monkeys(input, inspection_divider);
    let mut pending_throws: [VecDeque<i32>; MAX_MONKEYS] = Default::default();
    let mut throw_counter = [0; MAX_MONKEYS];

    let mut modulo = 1;
    for monkey in &monkeys {
        modulo = lcm(modulo, monkey.divisible_by);
    }

    for _ in 0..rounds {
        for (i, monkey) in monkeys.iter_mut().enumerate() {
            while !pending_throws[i].is_empty() {
                monkey.add_item(pending_throws[i].pop_front().unwrap());
            }

            while !monkey.items.is_empty() {
                let (value, target) = monkey.throw(modulo);
                throw_counter[i] += 1;
                pending_throws[target as usize].push_back(value);
            }
        }
    }

    throw_counter.sort();

    throw_counter[throw_counter.len() - 1] * throw_counter[throw_counter.len() - 2]
}

fn part1(input: &str) -> i64 {
    solve(input, 20, 3)
}

fn part2(input: &str) -> i64 {
    solve(input, 10000, 1)
}

#[test]
fn test1() {
    let result = part1(&get_input(DAY, true));
    assert_eq!(result, 10605);

    let result = part1(&get_input(DAY, false));
    assert_eq!(result, 54036);
}

#[test]
fn test2() {
    let result = part2(&get_input(DAY, true));
    assert_eq!(result, 2713310158);

    let result = part2(&get_input(DAY, false));
    assert_eq!(result, 13237873355);
}
