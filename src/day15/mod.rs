// https://adventofcode.com/2022/day/15

use crate::get_input;
use regex::Regex;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::FromStr;

const DAY: u8 = 15;
const MAX: i32 = 4_000_000;

pub fn main() {
    println!("{}", part1(&get_input(DAY, true), 10));
    println!("{}", part1(&get_input(DAY, false), 2_000_000));
    println!("{}", part2(&get_input(DAY, true), 29));
    println!("{}", part2(&get_input(DAY, false), MAX));
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

fn chord_range(radius: u32, sensor: &Point, tested_line: i32) -> Option<RangeInclusive<i32>> {
    let diameter = 2 * radius + 1;
    let tested_distance = sensor.y.abs_diff(tested_line);
    let chord_length = diameter.saturating_sub(2 * tested_distance);

    if chord_length == 0 {
        return None;
    }

    let half_chord = (chord_length as i32 - 1) / 2;
    let start = sensor.x - half_chord;
    let end = sensor.x + half_chord;
    Some(start..=end)
}

fn merge_ranges(ranges: &mut Vec<RangeInclusive<i32>>) -> Vec<RangeInclusive<i32>> {
    ranges.sort_unstable_by_key(|range| *range.start());
    let mut merged_ranges: Vec<RangeInclusive<i32>> = vec![ranges[0].clone()];

    'range: for range in ranges {
        for i in 0..merged_ranges.len() {
            if *range.start() <= merged_ranges[i].end() + 1
                && range.end() + 1 >= *merged_ranges[i].start()
            {
                let start = *range.start().min(merged_ranges[i].start());
                let end = *range.end().max(merged_ranges[i].end());
                merged_ranges[i] = start..=end;
                continue 'range;
            }
        }

        merged_ranges.push(range.clone());
    }

    merged_ranges
}

fn check_range(range: &RangeInclusive<i32>, tested_line: i32, max: i32) -> Option<Point> {
    if *range.start() > 0 {
        return Some(Point {
            x: range.start() - 1,
            y: tested_line,
        });
    }

    if *range.end() < max {
        return Some(Point {
            x: range.end() + 1,
            y: tested_line,
        });
    }

    None
}

fn part1(input: &str, tested_line: i32) -> u32 {
    let re = Regex::new(r"(-?\d+).*?(-?\d+).*?(-?\d+).*?(-?\d+)").unwrap();
    let mut result = 0;
    let mut found_beacons: HashSet<Point> = HashSet::new();
    let mut ranges: Vec<RangeInclusive<i32>> = vec![];
    let mut merged_ranges: Vec<RangeInclusive<i32>> = vec![];

    for line in input.lines() {
        let captures = re.captures_iter(line).last().unwrap();
        let x1 = i32::from_str(&captures[1]).unwrap();
        let y1 = i32::from_str(&captures[2]).unwrap();
        let x2 = i32::from_str(&captures[3]).unwrap();
        let y2 = i32::from_str(&captures[4]).unwrap();

        let sensor = Point { x: x1, y: y1 };
        let beacon = Point { x: x2, y: y2 };

        if sensor.y == tested_line && !found_beacons.contains(&sensor) {
            result -= 1;
            found_beacons.insert(sensor.clone());
            ranges.push(sensor.x..=sensor.x);
        }
        if beacon.y == tested_line && sensor != beacon && !found_beacons.contains(&beacon) {
            result -= 1;
            found_beacons.insert(beacon.clone());
            ranges.push(beacon.x..=beacon.x);
        }

        let radius = sensor.distance(&beacon);
        if sensor.y.abs_diff(tested_line) <= radius {
            let chord_range = chord_range(radius, &sensor, tested_line);
            ranges.push(chord_range.unwrap());
        }
    }

    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    for range in &ranges {
        if merged_ranges.is_empty() {
            merged_ranges.push(range.clone());
            continue;
        }
        for i in 0..merged_ranges.len() {
            let merged_range = &merged_ranges[i];

            if range.start() <= merged_range.end() && range.end() >= merged_range.start() {
                let start = *range.start().min(merged_range.start());
                let end = *range.end().max(merged_range.end());
                let new_range = start..=end;
                merged_ranges[i] = new_range;
            }
        }
    }

    for range in merged_ranges {
        result += range.end() - range.start() + 1;
    }

    result as u32
}

fn part2(input: &str, max: i32) -> u64 {
    let re = Regex::new(r"(-?\d+).*?(-?\d+).*?(-?\d+).*?(-?\d+)").unwrap();
    let mut sensors_and_beacons: Vec<(Point, Point, u32)> = vec![];
    let mut empty_point = Point { x: 0, y: 0 };
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;

    for line in input.lines() {
        let captures = re.captures_iter(line).last().unwrap();
        let x1 = i32::from_str(&captures[1]).unwrap();
        let y1 = i32::from_str(&captures[2]).unwrap();
        let x2 = i32::from_str(&captures[3]).unwrap();
        let y2 = i32::from_str(&captures[4]).unwrap();

        let sensor = Point { x: x1, y: y1 };
        let beacon = Point { x: x2, y: y2 };
        let radius = sensor.distance(&beacon);

        // A small optimization.
        min_x = min_x.min(sensor.x).max(0);
        max_x = max_x.max(sensor.x).min(MAX);

        sensors_and_beacons.push((sensor, beacon, radius));
    }

    'lines: for tested_line in (min_x..max_x).rev() {
        let mut found_beacons: HashSet<Point> = HashSet::new();
        let mut ranges: Vec<RangeInclusive<i32>> = vec![];

        for sensor_and_beacon in &sensors_and_beacons {
            let sensor = &sensor_and_beacon.0;
            let beacon = &sensor_and_beacon.1;
            let radius = sensor_and_beacon.2;

            if sensor.y == tested_line && !found_beacons.contains(sensor) {
                found_beacons.insert(sensor.clone());
                ranges.push(sensor.x..=sensor.x);
            }
            if beacon.y == tested_line && sensor != beacon && !found_beacons.contains(beacon) {
                found_beacons.insert(beacon.clone());
                ranges.push(beacon.x..=beacon.x);
            }

            let chord_range = chord_range(radius, sensor, tested_line);
            if let Some(value) = chord_range {
                ranges.push(value);
            }
        }

        ranges = merge_ranges(&mut ranges);

        if ranges.len() > 1 {
            for range in ranges {
                let check = check_range(&range, tested_line, max);
                if check.is_some() {
                    empty_point = check.unwrap();
                    break 'lines;
                }
            }
        }
    }

    dbg!(&empty_point);
    empty_point.x as u64 * MAX as u64 + empty_point.y as u64
}

#[test]
fn test1() {
    let result = part1(&get_input(DAY, true), 10);
    assert_eq!(result, 26);

    let result = part1(&get_input(DAY, false), 2_000_000);
    assert_eq!(result, 4919281);
}

#[test]
fn test2() {
    let result = part2(&get_input(DAY, true), 20);
    dbg!(result);
    assert_eq!(result, 56000011);

    let result = part2(&get_input(DAY, false), MAX);
    dbg!(result);
    assert_eq!(result, 12630143363767);
}
