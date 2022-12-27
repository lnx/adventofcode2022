use std::fs;

const ADJACENT: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
const MOVES: [[(i32, i32); 3]; 4] = [
    [(-1, -1), (0, -1), (1, -1)],
    [(-1, 1), (0, 1), (1, 1)],
    [(-1, -1), (-1, 0), (-1, 1)],
    [(1, -1), (1, 0), (1, 1)]
];

fn parse(input: &str) -> Vec<(i32, i32)> {
    input.lines().enumerate().flat_map(|(y, l)| {
        l.chars().enumerate().filter(|(_, c)| *c == '#')
            .map(|(x, _)| (x as i32, y as i32)).collect::<Vec<_>>()
    }).collect()
}

fn is_candidate(positions: &Vec<(i32, i32)>, x: i32, y: i32) -> bool {
    ADJACENT.iter().any(|(dx, dy)| positions.contains(&(x + dx, y + dy)))
}

fn simulate(mut positions: Vec<(i32, i32)>, round: usize) -> (Vec<(i32, i32)>, usize) {
    for r in 0..round {
        let candidates = positions.iter().enumerate()
            .filter(|(_, &(x, y))| is_candidate(&positions, x, y))
            .map(|(i, _)| i).collect::<Vec<_>>();
        let first_proposes = candidates.iter().map(|&i| {
            let (x, y) = positions[i];
            for i in r..r + MOVES.len() {
                if MOVES[i % MOVES.len()].iter().all(|&(dx, dy)| !positions.contains(&(x + dx, y + dy))) {
                    return Some(match i % MOVES.len() {
                        0 => (x, y - 1), // north
                        1 => (x, y + 1), // south
                        2 => (x - 1, y), // west
                        3 => (x + 1, y), // east
                        _ => unreachable!(),
                    });
                }
            }
            None
        }).collect::<Vec<_>>();
        let second_proposes = first_proposes.iter().enumerate().map(|(i, &p)| {
            if first_proposes.iter().enumerate().any(|(j, &other)| i != j && p == other) {
                None
            } else {
                p
            }
        }).collect::<Vec<_>>();
        if second_proposes.iter().all(|p| p.is_none()) {
            return (positions, r + 1);
        }
        candidates.into_iter().zip(second_proposes.into_iter()).for_each(|(i, p)| {
            if let Some(pos) = p {
                positions[i] = pos;
            }
        });
    }
    (positions, round)
}

#[allow(dead_code)]
fn print(positions: &Vec<(i32, i32)>, round: usize) {
    println!("== End of Round {} ==", round);
    let min_x = positions.iter().map(|&(x, _)| x).min().unwrap();
    let max_x = positions.iter().map(|&(x, _)| x).max().unwrap();
    let min_y = positions.iter().map(|&(_, y)| y).min().unwrap();
    let max_y = positions.iter().map(|&(_, y)| y).max().unwrap();
    for y in min_y..=max_y {
        let mut row = vec![];
        for x in min_x..=max_x {
            row.push(if !positions.contains(&(x, y)) { '.' } else { '#' });
        }
        println!("{}", String::from_iter(row));
    }
}

#[allow(dead_code)]
fn print_xy(positions: &Vec<(i32, i32)>, limit: usize) {
    println!("{}", positions.iter().take(limit).map(|(x, y)| format!("({},{})", x, y)).collect::<Vec<_>>().join(" "));
}

fn puzzle1(input: &str) -> usize {
    let (positions, _) = simulate(parse(input), 10);
    let min_x = positions.iter().map(|&(x, _)| x).min().unwrap();
    let max_x = positions.iter().map(|&(x, _)| x).max().unwrap();
    let min_y = positions.iter().map(|&(_, y)| y).min().unwrap();
    let max_y = positions.iter().map(|&(_, y)| y).max().unwrap();
    let mut res = 0;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if !positions.contains(&(x, y)) {
                res += 1;
            }
        }
    }
    res
}

fn puzzle2(input: &str) -> usize {
    let (_, round) = simulate(parse(input), usize::MAX);
    round
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(&INPUT), 110);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(&INPUT), 20);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("puzzle1:{:?}", puzzle1(&input));
    println!("puzzle2:{:?}", puzzle2(&input));
}