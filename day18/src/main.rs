use std::collections::HashSet;
use std::fs;

fn puzzle1(input: &str) -> usize {
    let drops = parse(input);
    drops.iter().flat_map(|&d| sides(d)).filter(|s| !drops.contains(s)).count()
}

fn puzzle2(input: &str) -> usize {
    let drops = parse(input);
    let min = drops.iter().flat_map(|(x, y, z)| [x, y, z]).min().unwrap() - 1;
    let max = drops.iter().flat_map(|(x, y, z)| [x, y, z]).max().unwrap() + 1;
    let mut stack = vec![(0, 0, 0)];
    let mut seen = HashSet::new();
    while let Some(p) = stack.pop() {
        for s in sides(p) {
            if !drops.contains(&s) && !seen.contains(&s)
                && [s.0, s.1, s.2].iter().all(|&i| min <= i && i <= max) {
                stack.push(s);
                seen.insert(s);
            }
        }
    }
    drops.iter().flat_map(|&d| sides(d)).filter(|s| seen.contains(s)).count()
}

fn parse(input: &str) -> HashSet<(i8, i8, i8)> {
    input.lines().map(|l| {
        let mut split = l.split(",");
        let x = split.next().unwrap().parse::<i8>().unwrap();
        let y = split.next().unwrap().parse::<i8>().unwrap();
        let z = split.next().unwrap().parse::<i8>().unwrap();
        (x, y, z)
    }).collect()
}

fn sides((x, y, z): (i8, i8, i8)) -> [(i8, i8, i8); 6] {
    [(x - 1, y, z), (x + 1, y, z), (x, y - 1, z), (x, y + 1, z), (x, y, z - 1), (x, y, z + 1)]
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(&INPUT), 64);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(&INPUT), 58);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("puzzle1:{:?}", puzzle1(&input));
    println!("puzzle2:{:?}", puzzle2(&input));
}