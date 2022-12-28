use std::fs;

fn encode(mut n: isize) -> String {
    let mut res = String::new();
    let mut carry = 0;
    loop {
        let (ch, carry_next) = match (n + carry) % 5 {
            0 => ('0', 0),
            1 => ('1', 0),
            2 => ('2', 0),
            3 => ('=', 1),
            4 => ('-', 1),
            _ => unreachable!(),
        };
        res.insert(0, ch);
        n = (n + carry) / 5;
        carry = carry_next;
        if n == 0 && carry == 0 {
            break;
        }
    }
    res
}

fn decode(s: &str) -> isize {
    s.chars().rev().enumerate().map(|(i, ch)| {
        isize::pow(5, i as u32) * match ch {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '=' => -2,
            '-' => -1,
            _ => unreachable!(),
        }
    }).sum()
}

fn puzzle1(input: &str) -> String {
    encode(input.lines().map(|l| decode(l)).sum())
}

fn puzzle2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(&INPUT), "2=-1=0");
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