# Advent of Code 2022

https://adventofcode.com/2022/about

# Template

```rust
use std::fs;

fn puzzle1(_input: &str) -> u32 {
    0
}

fn puzzle2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = "";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(&INPUT), 0);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(&INPUT), 0);
    }
}


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("puzzle1:{:?}", puzzle1(&input));
    println!("puzzle2:{:?}", puzzle2(&input));
}
```