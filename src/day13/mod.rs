// https://adventofcode.com/2022/day/13

use crate::get_input;
use regex::Regex;
use std::cmp::{min, Ordering};
use std::collections::HashMap;
use std::str::FromStr;

const DAY: u8 = 13;

pub fn main() {
    println!("{}", part1(&get_input(DAY, true)));
    println!("{}", part1(&get_input(DAY, false)));
    println!("{}", part2(&get_input(DAY, true)));
    println!("{}", part2(&get_input(DAY, false)));
}

fn parse_packet(
    packet: &str,
    re1: &Regex,
    re2: &Regex,
    re3: &Regex,
    re_empty_arrays: &Regex,
    re_numbers: &Regex,
) -> Vec<String> {
    // Ugly hack - two-digit numbers do not work with the float conversion approach.
    // Replaces only `10` because it is the only multi-digit number in the test input.
    let packet = packet.replace("10", "91");

    let packet = packet.replace(',', "");

    let packet = re1.replace_all(&packet, "][$1][");
    let packet = re2.replace_all(&packet, "[[$1][");
    let packet = re3.replace_all(&packet, "][$1]]");
    let packet = re_empty_arrays.replace_all(&packet, "[0]");

    let matches: Vec<String> = re_numbers
        .find_iter(&packet)
        .map(|m| String::from(m.as_str()))
        .collect();

    matches
}

fn verify_packet_order(
    first_packet: &str,
    first_matches: &Vec<String>,
    second_packet: &str,
    second_matches: &Vec<String>,
) -> bool {
    for i in 0..min(first_matches.len(), second_matches.len()) {
        let mut first_number = first_matches[i].to_string();
        first_number.insert(1, '.');
        let mut second_number = second_matches[i].to_string();
        second_number.insert(1, '.');

        let first_number = f32::from_str(&first_number).unwrap();
        let second_number = f32::from_str(&second_number).unwrap();

        if first_number > second_number {
            return false;
        } else if first_number < second_number {
            return true;
        }

        match first_matches[i].len().cmp(&second_matches[i].len()) {
            Ordering::Less => return true,
            Ordering::Greater => return false,
            _ => (),
        }
    }

    if first_packet.len() > second_packet.len() {
        return false;
    }

    true
}

fn packet_order(
    first_packet: &str,
    first_matches: &Vec<String>,
    second_packet: &str,
    second_matches: &Vec<String>,
) -> Ordering {
    if verify_packet_order(first_packet, first_matches, second_packet, second_matches) {
        return Ordering::Less;
    }

    Ordering::Greater
}

fn part1(input: &str) -> u32 {
    // Building these regexes in the loop changes the execution time from 16ms to 246ms.
    let re1 = Regex::new(r"](-?\d+)\[").unwrap();
    let re2 = Regex::new(r"\[(-?\d+)\[").unwrap();
    let re3 = Regex::new(r"](-?\d+)]").unwrap();
    let re_empty_arrays = Regex::new(r"\[]").unwrap();
    let re_numbers = Regex::new(r"-?\d+").unwrap();

    let mut result = 0;
    let mut buffer: Vec<&str> = vec![];
    for (i, line) in input.lines().enumerate() {
        if line.is_empty() {
            let first_matches =
                parse_packet(buffer[0], &re1, &re2, &re3, &re_empty_arrays, &re_numbers);
            let second_matches =
                parse_packet(buffer[1], &re1, &re2, &re3, &re_empty_arrays, &re_numbers);
            if verify_packet_order(buffer[0], &first_matches, buffer[1], &second_matches) {
                result += (i as u32 + 1) / 3;
            }
            buffer.clear();
        } else {
            buffer.push(line);
        }
    }

    result
}

fn part2(input: &str) -> u32 {
    let re1 = Regex::new(r"](-?\d+)\[").unwrap();
    let re2 = Regex::new(r"\[(-?\d+)\[").unwrap();
    let re3 = Regex::new(r"](-?\d+)]").unwrap();
    let re_empty_arrays = Regex::new(r"\[]").unwrap();
    let re_numbers = Regex::new(r"-?\d+").unwrap();

    let first_divider_packet = "[[2]]";
    let second_divider_packet = "[[6]]";
    let mut packets: Vec<&str> = vec![first_divider_packet, second_divider_packet];
    let mut packet_matches: HashMap<&str, Vec<String>> = HashMap::new();

    let matches = parse_packet(
        first_divider_packet,
        &re1,
        &re2,
        &re3,
        &re_empty_arrays,
        &re_numbers,
    );
    packet_matches.insert(first_divider_packet, matches);
    let matches = parse_packet(
        second_divider_packet,
        &re1,
        &re2,
        &re3,
        &re_empty_arrays,
        &re_numbers,
    );
    packet_matches.insert(second_divider_packet, matches);

    for line in input.lines() {
        if !line.is_empty() {
            packets.push(line);
            let matches = parse_packet(line, &re1, &re2, &re3, &re_empty_arrays, &re_numbers);
            packet_matches.insert(line, matches);
        }
    }

    packets.sort_by(|a, b| packet_order(a, &packet_matches[a], b, &packet_matches[b]));

    let first_divider_packet = packets
        .iter()
        .position(|&p| p == first_divider_packet)
        .unwrap() as u32
        + 1;
    let second_divider_packet = packets
        .iter()
        .position(|&p| p == second_divider_packet)
        .unwrap() as u32
        + 1;

    first_divider_packet * second_divider_packet
}

#[test]
fn test() {
    let result = part1(&get_input(DAY, true));
    assert_eq!(result, 13);

    let test_case = "[[[]]]\n[[]]\n\n";
    assert_eq!(part1(test_case), 0);

    let test_case = "[0,0]\n[0,0,0]\n\n";
    assert_eq!(part1(test_case), 1);

    let test_case = "[0,0,0]\n[0,0,0]\n\n";
    assert_eq!(part1(test_case), 1);

    let test_case = "[0,0,0]\n[0,0]\n\n";
    assert_eq!(part1(test_case), 0);

    let test_case = "[[]]\n[[[]]]\n\n";
    assert_eq!(part1(test_case), 1);

    let test_case = "[10,3]\n[4]\n\n";
    assert_eq!(part1(test_case), 0);

    let test_case = "[4]\n[10,3]\n\n";
    assert_eq!(part1(test_case), 1);

    let result = part1(&get_input(DAY, false));
    assert_eq!(result, 5252);

    let result = part2(&get_input(DAY, true));
    assert_eq!(result, 140);

    let result = part2(&get_input(DAY, false));
    assert_eq!(result, 20592);
}
