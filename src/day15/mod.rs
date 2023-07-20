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

fn check_point(point: &Point, sensors_and_beacons: &Vec<(Point, u32)>) -> bool {
    // Check if the point is outside of the range of all sensors.
    for (sensor, distance) in sensors_and_beacons {
        if *distance >= sensor.distance(point) {
            return false;
        }
    }
    true
}

fn part2(input: &str, max: i32) -> u64 {
    let re = Regex::new(r"(-?\d+).*?(-?\d+).*?(-?\d+).*?(-?\d+)").unwrap();
    let mut sensors_with_distance: Vec<(Point, u32)> = vec![];
    let mut empty_point = Point { x: -1, y: -1 };

    for line in input.lines() {
        let captures = re.captures_iter(line).last().unwrap();
        let x1 = i32::from_str(&captures[1]).unwrap();
        let y1 = i32::from_str(&captures[2]).unwrap();
        let x2 = i32::from_str(&captures[3]).unwrap();
        let y2 = i32::from_str(&captures[4]).unwrap();

        let sensor = Point { x: x1, y: y1 };
        let beacon = Point { x: x2, y: y2 };
        let radius = sensor.distance(&beacon);

        sensors_with_distance.push((sensor, radius));
    }

    'main: for (sensor1, distance) in &sensors_with_distance {
        for (sensor2, distance2) in &sensors_with_distance {
            if sensor1 == sensor2 {
                continue;
            }

            // Generate 8 permutations of corners.
            for diff1 in [-1, 1] {
                for diff2 in [-1, 1] {
                    for diff_common in [-1, 1] {
                        let corner1 =
                            sensor1.x + sensor1.y * diff_common + (*distance as i32 + 1) * diff1;

                        let corner2 =
                            sensor2.x - sensor2.y * diff_common + (*distance2 as i32 + 1) * diff2;

                        let mut intersection_point = Point {
                            x: (corner1 + corner2) / 2,
                            y: corner1,
                        };
                        intersection_point.y -= intersection_point.x;

                        if intersection_point.x < 0
                            || intersection_point.y < 0
                            || intersection_point.x > max
                            || intersection_point.y > max
                        {
                            continue;
                        }

                        if check_point(&intersection_point, &sensors_with_distance) {
                            empty_point = intersection_point;
                            break 'main;
                        }
                    }
                }
            }
        }
    }

    if empty_point.x < 0 {
        panic!("You should implement _corner_ cases.")
    }
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
    assert_eq!(result, 56000011);

    let result = part2(&get_input(DAY, false), MAX);
    assert_eq!(result, 12630143363767);
}
