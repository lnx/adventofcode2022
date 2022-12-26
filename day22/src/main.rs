use std::fs;
use std::iter::repeat;

enum Move {
    Forward(u8),
    TurnRight,
    TurnLeft,
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let (map, path) = input.split_once("\n\n").unwrap();
    let mut map = map.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let width = map.iter().map(|row| row.len()).max().unwrap();
    map.iter_mut().for_each(|row| row.extend(repeat(' ').take(width - row.len())));
    let moves = path.replace("R", " R ").replace("L", " L ").split(" ").map(|s| {
        match s {
            "R" => Move::TurnRight,
            "L" => Move::TurnLeft,
            _ => {
                match s.parse::<u8>() {
                    Ok(steps) => Move::Forward(steps),
                    Err(_) => unreachable!(),
                }
            }
        }
    }).collect::<Vec<_>>();
    (map, moves)
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    row: usize,
    col: usize,
    dir: usize,
}

const DIR: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // 0:right, 1:down, 2:left, 3:ups

fn walk(map: &Vec<Vec<char>>, moves: &Vec<Move>, wrap: fn(map: &Vec<Vec<char>>, pos: Pos) -> Pos) -> usize {
    let mut pos = Pos { row: 0, col: map[0].iter().position(|&c| c == '.').unwrap(), dir: 0 };
    moves.iter().for_each(|m| {
        match m {
            Move::Forward(steps) => {
                for _ in 0..*steps {
                    let (dr, dc) = DIR[pos.dir];
                    let nr = (pos.row as isize + dr) as usize;
                    let nc = (pos.col as isize + dc) as usize;
                    match map.get(nr).and_then(|row| row.get(nc)).unwrap_or(&' ') {
                        '.' => (pos.row, pos.col) = (nr, nc),
                        '#' => break,
                        ' ' => {
                            let wrapped_pos = wrap(map, pos);
                            if map[wrapped_pos.row][wrapped_pos.col] == '#' {
                                break;
                            } else {
                                pos = wrapped_pos;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            Move::TurnRight => pos.dir = (pos.dir + 1) % 4,
            Move::TurnLeft => pos.dir = (pos.dir + 3) % 4,
        }
    });
    1000 * (pos.row + 1) + 4 * (pos.col + 1) + pos.dir
}

fn wrap1(map: &Vec<Vec<char>>, pos: Pos) -> Pos {
    let (mut pos, (dr, dc)) = (pos, DIR[pos.dir]);
    loop {
        pos.row = (pos.row as isize + dr).rem_euclid(map.len() as isize) as usize;
        pos.col = (pos.col as isize + dc).rem_euclid(map[0].len() as isize) as usize;
        if map[pos.row][pos.col] != ' ' {
            return pos;
        }
    }
}

fn puzzle1(input: &str) -> usize {
    let (map, moves) = parse(input);
    walk(&map, &moves, wrap1)
}

// ..1.
// 234.
// ..56
fn wrap2_test(_: &Vec<Vec<char>>, pos: Pos) -> Pos {
    let (qr, qc, nd) = match (pos.row / 4, pos.col / 4, pos.dir) {
        (0, 2, 0) => (2, 3, 2),
        (0, 2, 2) => (1, 1, 1),
        (0, 2, 3) => (1, 0, 1),
        (1, 0, 1) => (2, 2, 3),
        (1, 0, 2) => (2, 3, 3),
        (1, 0, 3) => (0, 2, 1),
        (1, 1, 1) => (2, 2, 0),
        (1, 1, 3) => (0, 2, 0),
        (1, 2, 0) => (2, 3, 1),
        (2, 2, 1) => (1, 0, 3),
        (2, 2, 2) => (1, 1, 3),
        (2, 3, 0) => (0, 2, 2),
        (2, 3, 1) => (1, 0, 0),
        (2, 3, 3) => (1, 2, 2),
        _ => unreachable!(),
    };
    let (dr, dc) = (pos.row % 4, pos.col % 4);
    let i = [dr, 3 - dc, 3 - dr, dc][pos.dir];
    let (nr, nc) = [(i, 0), (0, 3 - i), (3 - i, 3), (3, i)][nd];
    Pos { row: qr * 4 + nr, col: qc * 4 + nc, dir: nd }
}

// .12
// .3.
// 45.
// 6..
fn wrap2_prod(_: &Vec<Vec<char>>, pos: Pos) -> Pos {
    let (qr, qc, nd) = match (pos.row / 50, pos.col / 50, pos.dir) {
        (0, 1, 2) => (2, 0, 0),
        (0, 1, 3) => (3, 0, 0),
        (0, 2, 0) => (2, 1, 2),
        (0, 2, 1) => (1, 1, 2),
        (0, 2, 3) => (3, 0, 3),
        (1, 1, 0) => (0, 2, 3),
        (1, 1, 2) => (2, 0, 1),
        (2, 0, 2) => (0, 1, 0),
        (2, 0, 3) => (1, 1, 0),
        (2, 1, 0) => (0, 2, 2),
        (2, 1, 1) => (3, 0, 2),
        (3, 0, 0) => (2, 1, 3),
        (3, 0, 1) => (0, 2, 1),
        (3, 0, 2) => (0, 1, 1),
        _ => unreachable!(),
    };
    let (dr, dc) = (pos.row % 50, pos.col % 50);
    let i = [dr, 49 - dc, 49 - dr, dc][pos.dir];
    let (nr, nc) = [(i, 0), (0, 49 - i), (49 - i, 49), (49, i)][nd];
    Pos { row: qr * 50 + nr, col: qc * 50 + nc, dir: nd }
}

fn puzzle2(input: &str) -> usize {
    let (map, moves) = parse(input);
    if map.len() < 100 {
        walk(&map, &moves, wrap2_test)
    } else {
        walk(&map, &moves, wrap2_prod)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(&INPUT), 6032);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(&INPUT), 5031);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("puzzle1:{:?}", puzzle1(&input));
    println!("puzzle2:{:?}", puzzle2(&input));
}