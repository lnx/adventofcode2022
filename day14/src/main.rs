use std::cmp::{max, min};
use std::fs;

fn puzzle1(input: &str) -> usize {
    let (mut map, bottom) = parse(input);
    let mut units = 0;
    for _ in 0.. {
        let Some((x, y)) = find(&map, bottom + 2) else { break; };
        if y >= bottom {
            break;
        }
        map[y][x] = true;
        units += 1;
    }
    units
}

fn puzzle2(input: &str) -> u32 {
    let (mut map, bottom) = parse(input);
    let mut units = 0;
    for _ in 0.. {
        let Some((x, y)) = find(&map, bottom + 2) else { break; };
        map[y][x] = true;
        units += 1;
    }
    units
}

fn find(map: &Vec<Vec<bool>>, floor: usize) -> Option<(usize, usize)> {
    let (mut x, mut y): (usize, usize) = (500, 0);
    while y + 1 < floor {
        let Some(dx) = [0, -1, 1].into_iter().find(|&dx| !map[y + 1][(x as isize + dx) as usize]) else { break; };
        x = (x as isize + dx) as usize;
        y += 1;
    }
    if !map[y][x] { Some((x, y)) } else { None }
}

fn parse(input: &str) -> (Vec<Vec<bool>>, usize) {
    let mut map = vec![vec![false; 1000]; 1000]; // map[y][x]
    let mut bottom = 0;
    input.lines().for_each(|l| {
        let coordinates = l.split(" -> ").map(|p| {
            let (x, y) = p.split_once(',').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        }).collect::<Vec<_>>();
        coordinates.windows(2).for_each(|w| {
            let (x1, y1) = w[0];
            let (x2, y2) = w[1];
            for y in min(y1, y2)..=max(y1, y2) {
                for x in min(x1, x2)..=max(x1, x2) {
                    map[y][x] = true;
                }
            }
            bottom = max(bottom, max(y1, y2));
        })
    });
    (map, bottom)
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(&INPUT), 24);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(&INPUT), 93);
    }
}


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("puzzle1:{:?}", puzzle1(&input));
    println!("puzzle2:{:?}", puzzle2(&input));
}