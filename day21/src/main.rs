use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
enum Operation {
    Num(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

fn parse(input: &str) -> HashMap<String, Operation> {
    input.lines().map(|l| {
        let (name, monkey) = l.split_once(": ").unwrap();
        if let Ok(num) = monkey.parse::<i64>() {
            (name.to_string(), Operation::Num(num))
        } else {
            let mut split = monkey.split(' ');
            let a = split.next().unwrap().to_string();
            let o = split.next().unwrap();
            let b = split.next().unwrap().to_string();
            (name.to_string(), match o {
                "+" => Operation::Add(a, b),
                "-" => Operation::Sub(a, b),
                "*" => Operation::Mul(a, b),
                "/" => Operation::Div(a, b),
                _ => unreachable!(),
            })
        }
    }).collect()
}

fn dfs(monkeys: &HashMap<String, Operation>, name: &str) -> i64 {
    match monkeys.get(name).unwrap() {
        Operation::Num(num) => *num,
        Operation::Add(a, b) => dfs(monkeys, a) + dfs(monkeys, b),
        Operation::Sub(a, b) => dfs(monkeys, a) - dfs(monkeys, b),
        Operation::Mul(a, b) => dfs(monkeys, a) * dfs(monkeys, b),
        Operation::Div(a, b) => dfs(monkeys, a) / dfs(monkeys, b),
    }
}

fn puzzle1(input: &str) -> i64 {
    let monkeys = parse(input);
    dfs(&monkeys, "root")
}

fn puzzle2(input: &str) -> i64 {
    let mut monkeys = parse(input);
    monkeys.insert("root".to_string(), match monkeys.get("root").unwrap() {
        Operation::Add(a, b) => Operation::Sub(a.clone(), b.clone()),
        Operation::Sub(a, b) => Operation::Sub(a.clone(), b.clone()),
        Operation::Mul(a, b) => Operation::Sub(a.clone(), b.clone()),
        Operation::Div(a, b) => Operation::Sub(a.clone(), b.clone()),
        _ => unreachable!(),
    });

    if let Some(humn) = search(&mut monkeys, |v| v.cmp(&0)) {
        return humn;
    }
    if let Some(humn) = search(&mut monkeys, |v| 0.cmp(&v)) {
        return humn;
    }
    unreachable!()
}

fn search(monkeys: &mut HashMap<String, Operation>, cmp: fn(i64) -> Ordering) -> Option<i64> {
    let mut l = 0;
    let mut r = i64::MAX / 10000;
    while l < r {
        let m = (l + r) / 2;
        monkeys.insert("humn".to_string(), Operation::Num(m)).unwrap();
        let v = dfs(&monkeys, "root");
        match cmp(v) {
            Ordering::Equal => return Some(m),
            Ordering::Less => { l = m + 1; }
            Ordering::Greater => { r = m; }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(&INPUT), 152);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(&INPUT), 301);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("puzzle1:{:?}", puzzle1(&input));
    println!("puzzle2:{:?}", puzzle2(&input));
}