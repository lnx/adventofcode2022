use std::cmp::max;
use std::fs;

fn puzzle1(grid: &Vec<Vec<i8>>) -> u32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut res = 0;
    for i in 0..m {
        for j in 0..n {
            if i == 0 || i == m - 1 || j == 0 || j == n - 1 || is_visible(grid, i, j) {
                res += 1;
            }
        }
    }
    res
}

fn is_visible(grid: &Vec<Vec<i8>>, row: usize, col: usize) -> bool {
    let m = grid.len();
    let n = grid[0].len();
    let cur = grid[row][col];
    let res = (0..row).into_iter().all(|i| cur > grid[i][col])
        || (row + 1..m).into_iter().all(|i| cur > grid[i][col])
        || (0..col).into_iter().all(|j| cur > grid[row][j])
        || (col + 1..n).into_iter().all(|j| cur > grid[row][j]);
    res
}

fn puzzle2(grid: &Vec<Vec<i8>>) -> u32 {
    let mut res = 0;
    let m = grid.len();
    let n = grid[0].len();
    for i in 1..m - 1 {
        for j in 1..n - 1 {
            res = max(res, score(grid, i, j));
        }
    }
    return res;
}

fn score(grid: &Vec<Vec<i8>>, row: usize, col: usize) -> u32 {
    let m = grid.len();
    let n = grid[0].len();
    let cur = grid[row][col];

    let mut up = 0;
    for i in (0..row).rev() {
        up += 1;
        if cur <= grid[i][col] {
            break;
        }
    }

    let mut down = 0;
    for i in row + 1..m {
        down += 1;
        if cur <= grid[i][col] {
            break;
        }
    }

    let mut left = 0;
    for j in (0..col).rev() {
        left += 1;
        if cur <= grid[row][j] {
            break;
        }
    }

    let mut right = 0;
    for j in col + 1..n {
        right += 1;
        if cur <= grid[row][j] {
            break;
        }
    }

    up * down * left * right
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(&parse_input(&INPUT)), 21);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(&parse_input(&INPUT)), 8);
    }
}

fn parse_input(input: &str) -> Vec<Vec<i8>> {
    input.lines().map(|line| {
        line.chars().map(|c| c as i8 - '0' as i8).collect::<Vec<_>>()
    }).collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("puzzle1:{:?}", puzzle1(&parse_input(&input)));
    println!("puzzle2:{:?}", puzzle2(&parse_input(&input)));
}
