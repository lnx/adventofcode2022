extern crate core;

use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Sensor {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Beacon {
    x: i64,
    y: i64,
}

fn parse(input: &str) -> HashMap<Sensor, Beacon> {
    input.lines().map(|l| parse_pair(l)).collect()
}

fn parse_pair(input: &str) -> (Sensor, Beacon) {
    let (s, b) = input.trim_start_matches("Sensor at ").split_once(": closest beacon is at ").unwrap();
    let s = parse_position(s);
    let b = parse_position(b);
    (Sensor { x: s.0, y: s.1 }, Beacon { x: b.0, y: b.1 })
}

// x=1, y=2
fn parse_position(input: &str) -> (i64, i64) {
    let (x, y) = input.split_once(", ").unwrap();
    let x = x.trim_start_matches("x=").parse::<i64>().unwrap();
    let y = y.trim_start_matches("y=").parse::<i64>().unwrap();
    (x, y)
}

fn distance_manhattan(s: &Sensor, b: &Beacon) -> i64 {
    (s.x - b.x).abs() + (s.y - b.y).abs()
}

fn calculate_coverage(map: &HashMap<Sensor, Beacon>, y: i64) -> Vec<(i64, i64)> {
    let mut ranges = map.iter().flat_map(|(s, b)| {
        let distance_y = (s.y - y).abs();
        let distance_m = distance_manhattan(s, b);
        if distance_y <= distance_m {
            let distance_x = distance_m - distance_y;
            Some((s.x - distance_x, s.x + distance_x))
        } else {
            None
        }
    }).collect::<Vec<_>>();
    ranges.sort_unstable();


    let mut merged = vec![];
    let mut cur = ranges[0];
    for r in &ranges[1..] {
        if cur.1 >= r.0 {
            cur.1 = max(cur.1, r.1);
        } else {
            merged.push(cur);
            cur = (r.0, r.1);
        }
    }
    merged.push(cur);
    merged
}

fn puzzle1(input: &str, y: i64) -> i64 {
    let map = parse(input);
    let covered_ranges = calculate_coverage(&map, y);
    let beacons = map.iter().map(|(_, b)| b).filter(|b| b.y == y).collect::<HashSet<_>>().len();
    covered_ranges.iter().map(|r| r.1 - r.0 + 1).sum::<i64>() - beacons as i64
}

fn puzzle2(input: &str, upper_bound: i64) -> i64 {
    let map = parse(input);
    for y in 0..=upper_bound {
        let covered_ranges = calculate_coverage(&map, y);
        for r in &covered_ranges {
            if r.1 < 0 { // ignore
                continue;
            }
            if r.0 > upper_bound { // no result
                break;
            }
            if r.0 > 0 {
                return (r.0 - 1) * 4000000 + y;
            }
            if r.1 < upper_bound {
                return (r.1 + 1) * 4000000 + y;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(&INPUT, 10), 26);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(&INPUT, 20), 56000011);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("puzzle1:{:?}", puzzle1(&input, 2000000));
    println!("puzzle2:{:?}", puzzle2(&input, 4000000));
}