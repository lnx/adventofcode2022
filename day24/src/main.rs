use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Blizzard {
    r: u8,
    c: u8,
    d: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    blizzards: Vec<Blizzard>,
    cur: (u8, u8),
    dst: (u8, u8),
    rows: u8,
    cols: u8,
}

const DIR: [(i8, i8); 5] = [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)]; // up, down, left, right, wait

fn parse(input: &str) -> State {
    let (mut rows, mut cols) = (0, 0);
    let blizzards = input.lines().enumerate().flat_map(|(r, l)| {
        rows = (r + 1) as u8;
        l.chars().enumerate().flat_map(|(c, ch)| {
            cols = (c + 1) as u8;
            match ch {
                '^' => Some(Blizzard { r: r as u8, c: c as u8, d: 0 }),
                'v' => Some(Blizzard { r: r as u8, c: c as u8, d: 1 }),
                '<' => Some(Blizzard { r: r as u8, c: c as u8, d: 2 }),
                '>' => Some(Blizzard { r: r as u8, c: c as u8, d: 3 }),
                _ => None,
            }
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    State { blizzards, cur: (0, 1), dst: (rows - 1, cols - 2), rows, cols }
}

fn simulate(state: &mut State) {
    for b in &mut state.blizzards {
        let (dr, dc) = DIR[b.d as usize];
        let mut nr = (b.r as i16 + dr as i16) as u8;
        let mut nc = (b.c as i16 + dc as i16) as u8;
        if nr == state.rows - 1 {
            nr = 1;
        } else if nr == 0 {
            nr = state.rows - 2;
        }
        if nc == state.cols - 1 {
            nc = 1;
        } else if nc == 0 {
            nc = state.cols - 2;
        }
        b.r = nr;
        b.c = nc;
    }
}

fn is_wall(state: &State, r: u8, c: u8) -> bool {
    (r == 0 || r == state.rows - 1 || c == 0 || c == state.cols - 1)
        && (r, c) != (0, 1) // src
        && (r, c) != (state.rows - 1, state.cols - 2) // dst
}

fn is_blizzard(blizzards: &Vec<Blizzard>, nr: u8, nc: u8) -> bool {
    blizzards.iter().any(|b| b.r == nr && b.c == nc)
}

fn bfs(state: State) -> (usize, State) {
    let mut deque = VecDeque::new();
    let mut seen = HashSet::new();
    deque.push_back((0, state));
    while let Some((steps, mut state)) = deque.pop_front() {
        if state.cur == state.dst {
            return (steps, state);
        }
        if seen.contains(&state) {
            continue;
        }
        seen.insert(state.clone());
        simulate(&mut state);
        for (dr, dc) in DIR {
            let nr = (state.cur.0 as i16 + dr as i16) as u8;
            let nc = (state.cur.1 as i16 + dc as i16) as u8;
            if nr < state.rows && nc < state.cols
                && !is_wall(&state, nr, nc) && !is_blizzard(&state.blizzards, nr, nc) {
                let mut next = state.clone();
                next.cur = (nr, nc);
                deque.push_back((steps + 1, next));
            }
        }
    }
    unreachable!()
}

fn puzzle1(input: &str) -> usize {
    let (steps, _) = bfs(parse(input));
    steps
}

fn puzzle2(input: &str) -> usize {
    let mut res = 0;

    // first trip
    let (steps, mut state) = bfs(parse(input));
    res += steps;

    // back to the start
    state.dst = (0, 1);
    let (steps, mut state) = bfs(state);
    res += steps;

    // back to the goal
    state.dst = (state.rows - 1, state.cols - 2);
    let (steps, _) = bfs(state);
    res += steps;

    res
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(&INPUT), 18);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(&INPUT), 54);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("puzzle1:{:?}", puzzle1(&input));
    println!("puzzle2:{:?}", puzzle2(&input));
}