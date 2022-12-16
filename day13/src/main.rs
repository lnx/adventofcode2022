// ref: https://github.com/AxlLind/AdventOfCode2022/blob/main/src/bin/13.rs

use std::cmp::{max, Ordering};
use std::fs;

use serde_json::Value;

fn puzzle1(input: &str) -> usize {
    let signals = input.lines().filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str::<Value>(l).unwrap()).collect::<Vec<_>>();
    signals.chunks(2).enumerate().map(|(i, ck)| {
        match cmp(&ck[0], &ck[1]) {
            Ordering::Greater => 0,
            _ => i + 1,
        }
    }).sum()
}

fn puzzle2(input: &str) -> usize {
    let mut signals = input.lines().filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str::<Value>(l).unwrap()).collect::<Vec<_>>();
    let dividers = [
        serde_json::from_str::<Value>("[[2]]").unwrap(),
        serde_json::from_str::<Value>("[[6]]").unwrap(),
    ];
    signals.extend(dividers.iter().cloned());
    signals.sort_unstable_by(|a, b| cmp(a, b));
    signals.iter().enumerate().map(|(i, v)| { if dividers.contains(v) { i + 1 } else { 1 } })
        .fold(1, |acc, item| acc * item)
}

fn cmp(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => x.as_u64().unwrap().cmp(&y.as_u64().unwrap()),
        (Value::Array(a), Value::Array(b)) => {
            for i in 0..max(a.len(), b.len()) {
                match (a.get(i), b.get(i)) {
                    (None, _) => return Ordering::Less,
                    (_, None) => return Ordering::Greater,
                    (Some(x), Some(y)) => match cmp(x, y) {
                        Ordering::Equal => {} // continue
                        c => return c,
                    }
                }
            }
            Ordering::Equal
        }
        (Value::Array(_), Value::Number(_)) => cmp(a, &Value::Array(vec![b.clone()])),
        (Value::Number(_), Value::Array(_)) => cmp(&Value::Array(vec![a.clone()]), b),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(&INPUT), 13);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(&INPUT), 140);
    }
}


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("puzzle1:{:?}", puzzle1(&input));
    println!("puzzle2:{:?}", puzzle2(&input));
}