use std::collections::VecDeque;
use std::fs;

fn parse_input(input: &str) -> (Vec<VecDeque<char>>, Vec<Vec<usize>>) {
    let mut split = input.split("\n\n");
    let stacks = split.next().unwrap();
    let stacks = stacks.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let stacks = stacks[stacks.len() - 1].iter().enumerate()
        .filter(|(_, &c)| c.is_digit(10))
        .map(|(j, _)| {
            (0..stacks.len() - 1).rev()
                .flat_map(|i| stacks[i].get(j))
                .filter(|c| c.is_ascii_alphabetic())
                .copied()
                .collect::<VecDeque<_>>()
        })
        .collect::<Vec<_>>();

    let moves = split.next().unwrap(); // (num, from, to)
    let moves = moves.lines().map(|line| {
        line.split(' ').skip(1).step_by(2).map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    (stacks, moves)
}

fn puzzle1(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);
    moves.iter().for_each(|m| {
        (0..m[0]).for_each(|_| {
            let pop = stacks[m[1] - 1].pop_back().unwrap();
            stacks[m[2] - 1].push_back(pop);
        })
    });
    let mut res = String::new();
    for i in 0..stacks.len() {
        res.push(stacks[i].pop_back().unwrap());
    }
    res
}

fn puzzle2(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);
    let mut temp = VecDeque::new();
    moves.iter().for_each(|m| {
        (0..m[0]).for_each(|_| {
            let pop = stacks[m[1] - 1].pop_back().unwrap();
            temp.push_back(pop);
        });
        (0..m[0]).for_each(|_| {
            let pop = temp.pop_back().unwrap();
            stacks[m[2] - 1].push_back(pop);
        });
    });
    let mut res = String::new();
    for i in 0..stacks.len() {
        res.push(stacks[i].pop_back().unwrap());
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(INPUT), "CMZ".to_string());
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(INPUT), "MCD".to_string());
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("puzzle1:{:?}", puzzle1(&input));
    println!("puzzle2:{:?}", puzzle2(&input));
}
