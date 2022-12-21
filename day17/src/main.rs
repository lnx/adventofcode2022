// ref: https://github.com/AxlLind/AdventOfCode2022/blob/main/src/bin/17.rs

use std::collections::HashMap;
use std::fs;

const ROCKS_LIST: [&[(usize, usize)]; 5] = [
    &[(0, 0), (0, 1), (0, 2), (0, 3)],
    &[(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
    &[(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
    &[(0, 0), (1, 0), (2, 0), (3, 0)],
    &[(0, 0), (0, 1), (1, 0), (1, 1)],
];

fn get_height(map: &[[u8; 7]]) -> usize {
    map.iter().position(|row| row == &[0; 7]).unwrap()
}

fn can_fit(map: &[[u8; 7]], rocks: &[(usize, usize)], h: usize, w: usize) -> bool {
    rocks.iter().all(|(dh, dw)| w + dw < 7 && map[h + dh][w + dw] != b'#')
}

// used as key
fn skyline(map: &[[u8; 7]]) -> [usize; 7] {
    let mut res = [0; 7];
    let h = get_height(map);
    for i in 0..7 {
        res[i] = (0..h).find(|&x| map[h - x - 1][i] == b'#').unwrap_or(usize::MAX);
    }
    res
}

fn simulate(input: &str, rocks_limit: usize) -> usize {
    let mut map = [[0; 7]; 100000];
    let mut i = 0; // rocks index
    let mut j = 0; // move index, jets of hot gas
    let mut cache = HashMap::new(); // key -> (i, height)
    let mut repeated_height = 0;
    while i < rocks_limit {
        let rocks = ROCKS_LIST[i % ROCKS_LIST.len()];
        let (mut h, mut w) = (get_height(&map) + 3, 2); // init pos
        loop {
            match input.as_bytes()[j % input.len()] {
                b'>' => if can_fit(&map, rocks, h, w + 1) { w += 1 },
                b'<' => if w > 0 && can_fit(&map, rocks, h, w - 1) { w -= 1 },
                _ => unreachable!(),
            }
            j += 1;
            if h == 0 || !can_fit(&map, rocks, h - 1, w) {
                break;
            }
            h -= 1;
        }
        for (dh, dw) in rocks.iter() {
            map[h + dh][w + dw] = b'#';
        }
        let key = (i % ROCKS_LIST.len(), j % input.len(), skyline(&map));
        if let Some((prev_index, prev_height)) = cache.get(&key) {
            let mut repeats = (rocks_limit - prev_index) / (i - prev_index) - 1;
            if i + (i - prev_index) * repeats >= rocks_limit {
                repeats -= 1; // take care of boundary
            }
            i += (i - prev_index) * repeats;
            repeated_height += (get_height(&map) - prev_height) * repeats;
        } else {
            cache.insert(key, (i, get_height(&map)));
        }
        i += 1;
        // println!("{}\n", display(&map));
    }
    repeated_height + get_height(&map)
}

#[allow(dead_code)]
fn display(map: &[[u8; 7]]) -> String {
    let h = get_height(map);
    let mut lines = (0..h)
        .map(|i| map[i].iter().map(|&c| if c == 0 { '.' } else { '#' }).collect::<String>())
        .map(|l| format!("|{}|", l))
        .collect::<Vec<_>>();
    lines.insert(0, "+-------+".to_string());
    lines.reverse();
    lines.join("\n")
}

fn puzzle1(input: &str) -> usize {
    simulate(input, 2022)
}

fn puzzle2(input: &str) -> usize {
    simulate(input, 1_000_000_000_000)
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(&INPUT), 3068);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(&INPUT), 1514285714288);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("puzzle1:{:?}", puzzle1(&input));
    println!("puzzle2:{:?}", puzzle2(&input));
}