use std::collections::HashSet;
use std::fs;

fn puzzle1(input: &str) -> u32 {
    input.lines().map(|line| {
        let items: Vec<_> = line.chars().collect::<Vec<_>>();
        let (first, second) = items.split_at(items.len() / 2);
        let first: HashSet<_> = first.iter().copied().collect();
        let second: HashSet<_> = second.iter().copied().collect();
        priority(first.intersection(&second).copied())
    }).sum()
}

fn puzzle2(input: &str) -> u32 {
    input.lines().collect::<Vec<_>>().chunks_exact(3) // no remainder
        .map(|group| {
            let badge = group.iter()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .fold(None, |acc: Option<HashSet<char>>, hs| {
                    if let Some(acc) = acc {
                        Some(acc.intersection(&hs).copied().collect())
                    } else {
                        Some(hs)
                    }
                }).unwrap();
            priority(badge.into_iter())
        }).sum()
}

fn priority(items: impl Iterator<Item=char>) -> u32 {
    items.map(|item| {
        let p = match item {
            'a'..='z' => item as u8 - 'a' as u8 + 1,
            'A'..='Z' => item as u8 - 'A' as u8 + 27,
            _ => unreachable!(),
        };
        p as u32
    }).sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT1: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    const INPUT2: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(INPUT1), 157);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(INPUT2), 70);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("puzzle1:{:?}", puzzle1(&input));
    println!("puzzle2:{:?}", puzzle2(&input));
}
