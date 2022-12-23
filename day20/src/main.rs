extern crate core;

use std::fs;

#[derive(Debug, Clone, Copy)]
struct Num {
    id: i64,
    val: i64,
}

fn parse(input: &str, key: i64) -> Vec<Num> {
    let mut id = 0;
    input.lines().map(|l| {
        id += 1;
        Num { id, val: l.parse::<i64>().unwrap() * key }
    }).collect()
}

fn mix(nums: Vec<Num>, iterations: i64) -> i64 {
    let mut ans = nums.clone();
    for _ in 0..iterations {
        for num in &nums {
            let pos = ans.iter().position(|x| x.id == num.id).unwrap();
            let num = ans.remove(pos);
            let pos = (pos as i64 + num.val).rem_euclid(ans.len() as i64) as usize;
            ans.insert(pos, num);
        }
    }
    let pos = ans.iter().position(|&x| x.val == 0).unwrap();
    [1000, 2000, 3000].iter().map(|&nth| ans[(pos + nth) % ans.len()].val).sum()
}

fn puzzle1(input: &str) -> i64 {
    mix(parse(input, 1), 1)
}

fn puzzle2(input: &str) -> i64 {
    mix(parse(input, 811589153), 10)
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = "1
2
-3
3
-2
0
4";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(&INPUT), 3);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(&INPUT), 1623178306);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("puzzle1:{:?}", puzzle1(&input));
    println!("puzzle2:{:?}", puzzle2(&input));
}