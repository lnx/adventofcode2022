#![feature(test)]
extern crate test;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;

fn puzzle1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|group| group.lines().map(|s| s.parse::<u32>().unwrap()).sum::<u32>())
        .max()
        .unwrap()
}

fn puzzle2(input: &str) -> u32 {
    let mut calories: Vec<_> = input
        .split("\n\n")
        .map(|group| group.lines().map(|s| s.parse::<u32>().unwrap()).sum::<u32>())
        .collect();
    calories.sort_by(|a, b| b.cmp(a));
    calories.into_iter().take(3).sum()
}

fn puzzle2_heap(input: &str) -> u32 {
    let mut heap = BinaryHeap::new();
    input
        .split("\n\n")
        .map(|group| group.lines().map(|s| s.parse::<u32>().unwrap()).sum::<u32>())
        .for_each(|caloric| {
            if heap.len() < 3 {
                heap.push(Reverse(caloric));
            } else if heap.peek().unwrap().0 < caloric {
                heap.pop();
                heap.push(Reverse(caloric));
            }
        });
    (0..3).flat_map(|_| heap.pop().map(|r| r.0)).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use test::Bencher;

    use crate::*;

    const INPUT: &'static str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(INPUT), 24000);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(INPUT), 45000);
    }

    #[test]
    fn test_puzzle2_heap() {
        assert_eq!(puzzle2(INPUT), 45000);
    }

    #[bench]
    fn bench_puzzle1(b: &mut Bencher) {
        let input = fs::read_to_string("input.txt").unwrap();
        b.iter(|| {
            puzzle1(&input)
        });
    }

    #[bench]
    fn bench_puzzle2(b: &mut Bencher) {
        let input = fs::read_to_string("input.txt").unwrap();
        b.iter(|| {
            puzzle2(&input)
        });
    }

    #[bench]
    fn bench_puzzle2_heap(b: &mut Bencher) {
        let input = fs::read_to_string("input.txt").unwrap();
        b.iter(|| {
            puzzle2_heap(&input)
        });
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("puzzle1:{:?}", puzzle1(&input));
    println!("puzzle2:{:?}", puzzle2(&input));
    println!("puzzle2_heap:{:?}", puzzle2_heap(&input));
}
