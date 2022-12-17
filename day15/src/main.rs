extern crate core;

use std::cmp::max;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Sensor {
    sx: i64,
    sy: i64,
    bx: i64,
    by: i64,
    md: i64,
}

fn parse(input: &str) -> Vec<Sensor> {
    input.lines().map(|l| {
        let mut it = l.split(|c: char| !c.is_digit(10) && c != '-').filter_map(|w| w.parse::<i64>().ok());
        let sx = it.next().unwrap();
        let sy = it.next().unwrap();
        let bx = it.next().unwrap();
        let by = it.next().unwrap();
        let md = (sx - bx).abs() + (sy - by).abs();
        Sensor { sx, sy, bx, by, md }
    }).collect()
}

fn calculate_coverage(sensors: &Vec<Sensor>, y: i64) -> Vec<(i64, i64)> {
    let mut ranges = sensors.iter().flat_map(|s| {
        let left = s.md - (s.sy - y).abs();
        if left >= 0 { Some((s.sx - left, s.sx + left)) } else { None }
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
    let sensors = parse(input);
    let covered_ranges = calculate_coverage(&sensors, y);
    let beacons = sensors.iter().filter(|s| s.by == y).map(|s| (s.bx, s.by)).collect::<HashSet<_>>().len();
    covered_ranges.iter().map(|r| r.1 - r.0 + 1).sum::<i64>() - beacons as i64
}

fn puzzle2(input: &str, upper_bound: i64) -> i64 {
    let sensors = parse(input);
    for s in &sensors {
        for (dx, dy) in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
            for distance in 0..=(s.md + 1) {
                let bx = s.sx + dx * distance;
                let by = s.sy + dy * (s.md + 1 - distance);
                if bx < 0 || bx >= upper_bound || by < 0 || by >= upper_bound {
                    continue;
                }
                if sensors.iter().all(|s| (s.sx - bx).abs() + (s.sy - by).abs() > s.md) {
                    return bx * 4000000 + by;
                }
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