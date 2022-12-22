use std::cmp::max;
use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Resources {
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
    ore_bots: u16,
    clay_bots: u16,
    obsidian_bots: u16,
    geode_bots: u16,
}

impl Resources {
    fn collect(&mut self) {
        self.ore += self.ore_bots;
        self.clay += self.clay_bots;
        self.obsidian += self.obsidian_bots;
        self.geode += self.geode_bots;
    }

    fn try_build_geode(&self, blueprint: &Blueprint) -> Option<Self> {
        if self.ore >= blueprint.geode.0 && self.obsidian >= blueprint.geode.1 {
            let mut res = self.clone();
            res.ore -= blueprint.geode.0;
            res.obsidian -= blueprint.geode.1;
            res.collect();
            res.geode_bots += 1;
            Some(res)
        } else {
            None
        }
    }

    fn try_build_obsidian(&self, blueprint: &Blueprint) -> Option<Self> {
        if self.ore >= blueprint.obsidian.0 && self.clay >= blueprint.obsidian.1
            && self.obsidian_bots < blueprint.geode.1 {
            let mut res = self.clone();
            res.ore -= blueprint.obsidian.0;
            res.clay -= blueprint.obsidian.1;
            res.collect();
            res.obsidian_bots += 1;
            Some(res)
        } else {
            None
        }
    }

    fn try_build_clay(&self, blueprint: &Blueprint) -> Option<Self> {
        if self.ore >= blueprint.clay && self.clay_bots < blueprint.obsidian.1 {
            let mut res = self.clone();
            res.ore -= blueprint.clay;
            res.collect();
            res.clay_bots += 1;
            Some(res)
        } else {
            None
        }
    }

    fn try_build_ore(&self, blueprint: &Blueprint) -> Option<Self> {
        let max_ore_costs = *[blueprint.ore, blueprint.clay, blueprint.obsidian.0, blueprint.geode.0].iter().max().unwrap();
        if self.ore >= blueprint.ore && self.ore_bots < max_ore_costs {
            let mut res = self.clone();
            res.ore -= blueprint.ore;
            res.collect();
            res.ore_bots += 1;
            Some(res)
        } else {
            None
        }
    }
}

impl Default for Resources {
    fn default() -> Self {
        Resources {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_bots: 1,
            clay_bots: 0,
            obsidian_bots: 0,
            geode_bots: 0,
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    id: u16,
    ore: u16,
    clay: u16,
    obsidian: (u16, u16),
    geode: (u16, u16),
}

fn parse(input: &str) -> Vec<Blueprint> {
    input.replace("\n  Each", " Each").replace("\n\n", "\n").lines().map(|l| {
        let digits = l.chars().filter(|&c| c.is_ascii_digit() || c == ' ').collect::<String>();
        let digits = digits.split(' ').filter(|s| !s.is_empty()).flat_map(|s| s.parse::<u16>()).collect::<Vec<_>>();
        Blueprint {
            id: digits[0],
            ore: digits[1],
            clay: digits[2],
            obsidian: (digits[3], digits[4]),
            geode: (digits[5], digits[6]),
        }
    }).collect()
}

fn bfs(resources: Resources, time: u16, blueprint: &Blueprint) -> u16 {
    let mut deque = VecDeque::new();
    let mut seen = HashSet::new();
    deque.push_back((0, resources));

    let mut ans = 0;
    while let Some((steps, mut resources)) = deque.pop_front() {
        if steps >= time {
            ans = max(ans, resources.geode);
            // dbg!(resources);
            continue;
        }
        if seen.contains(&resources) {
            continue;
        }
        seen.insert(resources);

        if let Some(resources) = resources.try_build_geode(blueprint) {
            deque.push_back((steps + 1, resources));
            continue;
        }
        if let Some(resources) = resources.try_build_obsidian(blueprint) {
            deque.push_back((steps + 1, resources));
        }
        if let Some(resources) = resources.try_build_clay(blueprint) {
            deque.push_back((steps + 1, resources));
        }
        if let Some(resources) = resources.try_build_ore(blueprint) {
            deque.push_back((steps + 1, resources));
        }
        resources.collect();
        deque.push_back((steps + 1, resources));
    }
    ans
}

fn puzzle1(input: &str) -> u16 {
    parse(input).iter().map(|blueprint| {
        blueprint.id * bfs(Resources::default(), 24, blueprint)
    }).sum()
}

fn puzzle2(input: &str) -> u16 {
    parse(input).iter().take(3).map(|blueprint| {
        bfs(Resources::default(), 32, blueprint)
    }).fold(1, |acc, item| acc * item)
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = "Blueprint 1:
  Each ore robot costs 4 ore.
  Each clay robot costs 2 ore.
  Each obsidian robot costs 3 ore and 14 clay.
  Each geode robot costs 2 ore and 7 obsidian.

Blueprint 2:
  Each ore robot costs 2 ore.
  Each clay robot costs 3 ore.
  Each obsidian robot costs 3 ore and 8 clay.
  Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(&INPUT), 33);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(&INPUT), 3472);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("puzzle1:{:?}", puzzle1(&input));
    println!("puzzle2:{:?}", puzzle2(&input));
}