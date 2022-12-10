use std::collections::{HashMap, HashSet};
use std::fs;

fn puzzle1(input: &str) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    input.lines().flat_map(|line| {
        let mut split = line.split(' ');
        let mov = split.next().unwrap();
        let size = split.next().unwrap().parse::<u8>().unwrap();
        let dir = match mov {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => unreachable!(),
        };
        (0..size).map(|_| {
            head = (head.0 + dir.0, head.1 + dir.1); // new head
            tail = move_tail(head, tail);
            tail
        }).collect::<Vec<_>>()
    }).collect::<HashSet<_>>().len()
}

fn puzzle2(input: &str, initial: (i32, i32)) -> usize {
    let mut heads = [initial; 10];
    let tails = input.lines().flat_map(|line| {
        let mut split = line.split(' ');
        let mov = split.next().unwrap();
        let size = split.next().unwrap().parse::<u8>().unwrap();
        let dir = match mov {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => unreachable!(),
        };
        (0..size).map(|_| {
            heads[0] = (heads[0].0 + dir.0, heads[0].1 + dir.1); // new head
            for i in 1..heads.len() {
                heads[i] = move_tail(heads[i - 1], heads[i]);
            }
            let tail = heads[heads.len() - 1];
            tail
        }).collect::<Vec<_>>()
    }).collect::<HashSet<_>>();
    tails.len()
}

fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    if i32::abs(head.0 - tail.0) <= 1 && i32::abs(head.1 - tail.1) <= 1 { // adjacent
        return tail;
    }

    // dir_tail -> dir_heads
    let mut m = HashMap::new();
    m.insert((-1, 1), vec![(-2, 1), (-1, 2), (-2, 2)]);
    m.insert((0, 1), vec![(0, 2)]);
    m.insert((1, 1), vec![(1, 2), (2, 1), (2, 2)]);
    m.insert((1, 0), vec![(2, 0)]);
    m.insert((1, -1), vec![(2, -1), (1, -2), (2, -2)]);
    m.insert((0, -1), vec![(0, -2)]);
    m.insert((-1, -1), vec![(-1, -2), (-2, -1), (-2, -2)]);
    m.insert((-1, 0), vec![(-2, 0)]);

    for (dir_tail, dir_heads) in m {
        for dir_head in dir_heads {
            if tail.0 + dir_head.0 == head.0 && tail.1 + dir_head.1 == head.1 {
                return (tail.0 + dir_tail.0, tail.1 + dir_tail.1);
            }
        }
    }
    unreachable!()
}


#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT1: &'static str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT2: &'static str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(&INPUT1), 13);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(&INPUT1, (0, 0)), 1);
        assert_eq!(puzzle2(&INPUT2, (15, 11)), 36);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("puzzle1:{:?}", puzzle1(&input));
    println!("puzzle2:{:?}", puzzle2(&input, (0, 0)));
}