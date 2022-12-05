use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Pair {
    start: u32,
    end: u32,
}

impl Pair {
    fn contains(&self, other: &Pair) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    fn overlap(&self, other: &Pair) -> bool {
        !(self.end < other.start || self.start > other.end)
    }
}

impl FromStr for Pair {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('-');
        let start = split.next().unwrap().parse::<u32>()?;
        let end = split.next().unwrap().parse::<u32>()?;
        Ok(Self { start, end })
    }
}

fn puzzle1(input: &str) -> u32 {
    input.lines().map(|line| {
        let mut split = line.split(',');
        let a = split.next().unwrap().parse::<Pair>().unwrap();
        let b = split.next().unwrap().parse::<Pair>().unwrap();
        (a.contains(&b) || b.contains(&a)) as u32
    }).sum()
}

fn puzzle2(input: &str) -> u32 {
    input.lines().map(|line| {
        let mut split = line.split(',');
        let a = split.next().unwrap().parse::<Pair>().unwrap();
        let b = split.next().unwrap().parse::<Pair>().unwrap();
        a.overlap(&b) as u32
    }).sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(&INPUT), 2);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(&INPUT), 4);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("puzzle1:{:?}", puzzle1(&input));
    println!("puzzle2:{:?}", puzzle2(&input));
}
