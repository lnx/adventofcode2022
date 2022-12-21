// ref: https://www.youtube.com/watch?v=DgqkVDr1WX8&ab_channel=chrisbiscardi

use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Valve {
    name: String,
    flowrate: usize,
    tunnels: Vec<String>,
}

fn parse(input: &str) -> Vec<Valve> {
    let mut valves = input.lines().map(|l| {
        let (name, left) = l.trim_start_matches("Valve ").split_once(" has flow rate=").unwrap();
        let (flowrate, left) = left.split_once(";").unwrap();
        let flowrate = flowrate.parse::<usize>().unwrap();
        let tunnels = left.chars().filter(|c| c.is_ascii_alphabetic() && c.is_ascii_uppercase())
            .collect::<Vec<_>>().chunks(2).map(|ck| String::from_iter(ck)).collect();
        Valve { name: name.to_string(), flowrate, tunnels }
    }).collect::<Vec<_>>();
    valves.sort_unstable_by_key(|v| v.name.clone());
    valves
}

fn build(valves: &Vec<Valve>) -> (Vec<usize>, Vec<Vec<usize>>) {
    let lookup = valves.iter().enumerate().map(|(i, v)| (v.name.clone(), i)).collect::<HashMap<_, _>>();
    let mut flowrates = vec![0; valves.len()];
    let mut tunnels = vec![vec![]; valves.len()];
    for v in valves {
        let i = *lookup.get(&v.name).unwrap();
        flowrates[i] = v.flowrate;
        for name in &v.tunnels {
            let j = *lookup.get(name).unwrap();
            tunnels[i].push(j);
        }
    }
    (flowrates, tunnels)
}

fn dfs(cur: usize, opened: usize, minutes: usize, total_minutes: usize, other_players: usize,
       cache: &mut HashMap<(usize, usize, usize, usize), usize>, flowrates: &Vec<usize>, tunnels: &Vec<Vec<usize>>) -> usize {
    if minutes == 0 {
        return if other_players > 0 { // play it again
            dfs(0, opened, total_minutes, total_minutes, other_players - 1, cache, flowrates, tunnels)
        } else {
            0
        };
    }

    let key = (cur, opened, minutes, other_players);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    let mut res = 0;
    let closed = opened & (1 << cur) == 0;
    if closed && flowrates[cur] > 0 {
        let new_opened = opened | (1 << cur); // open this valve
        res = res.max((minutes - 1) * flowrates[cur] + dfs(cur, new_opened, minutes - 1, total_minutes, other_players, cache, flowrates, tunnels));
    }
    for &next in &tunnels[cur] {
        res = res.max(dfs(next, opened, minutes - 1, total_minutes, other_players, cache, flowrates, tunnels));
    }
    cache.insert(key, res);
    res
}

fn puzzle1(input: &str) -> usize {
    let (flowrates, tunnels) = build(&parse(input));
    dfs(0, 0, 30, 30, 0, &mut HashMap::new(), &flowrates, &tunnels)
}

fn puzzle2(input: &str) -> usize {
    let (flowrates, tunnels) = build(&parse(input));
    dfs(0, 0, 26, 26, 1, &mut HashMap::new(), &flowrates, &tunnels)
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(&INPUT), 1651);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(&INPUT), 1707);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("puzzle1:{:?}", puzzle1(&input));
    println!("puzzle2:{:?}", puzzle2(&input));
}