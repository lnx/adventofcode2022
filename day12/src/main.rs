use std::collections::{HashSet, VecDeque};
use std::fs;

fn puzzle1(input: &str) -> usize {
    let mut heights: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let start = find(&heights, 'S')[0];
    heights[start.0][start.1] = 'a';

    let end = find(&heights, 'E')[0];
    heights[end.0][end.1] = 'z';

    bfs(&heights, start, end)
}

fn puzzle2(input: &str) -> usize {
    let mut heights: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let start = find(&heights, 'S');
    for (row, col) in &start {
        heights[*row][*col] = 'a';
    }
    let start = find(&heights, 'a');

    let end = find(&heights, 'E')[0];
    heights[end.0][end.1] = 'z';

    start.iter().copied().map(|start| bfs(&heights, start, end)).min().unwrap()
}

fn bfs(heights: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> usize {
    let mut deque = VecDeque::new();
    let mut seen = HashSet::new();

    deque.push_back(start);
    seen.insert(start);
    let mut steps = 0;
    let mut found = false;
    while !deque.is_empty() && !found {
        steps += 1;
        for _ in 0..deque.len() {
            let pos = deque.pop_front().unwrap();
            if pos == end {
                found = true;
                break;
            }
            for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let next_row = pos.0 as isize + dir.0;
                let next_col = pos.1 as isize + dir.1;
                if next_row < 0 || next_row >= heights.len() as isize || next_col < 0
                    || next_col >= heights[0].len() as isize {
                    continue;
                }
                let next = (next_row as usize, next_col as usize);
                if heights[next.0][next.1] as i8 - heights[pos.0][pos.1] as i8 <= 1 && !seen.contains(&next) {
                    deque.push_back(next);
                    seen.insert(next);
                }
            }
        }
    }
    if found {
        steps - 1
    } else {
        usize::MAX
    }
}

fn find(heights: &Vec<Vec<char>>, target: char) -> Vec<(usize, usize)> {
    let mut res = vec![];
    for (row, chs) in heights.iter().enumerate() {
        for (col, &ch) in chs.iter().enumerate() {
            if ch == target {
                res.push((row, col));
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(&INPUT), 31);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(&INPUT), 29);
    }
}


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("puzzle1:{:?}", puzzle1(&input));
    println!("puzzle2:{:?}", puzzle2(&input));
}