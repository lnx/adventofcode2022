use std::cell::Cell;
use std::collections::{HashMap, VecDeque};
use std::fs;

struct Monkey {
    id: usize,
    items: Cell<VecDeque<u64>>,
    operation: Operation,
    divisor: u64,
    // false: index 0, true: index 1
    next: [usize; 2],
    inspected: Cell<u64>,
}

struct Operation {
    op: Box<dyn Fn(u64) -> u64>,
}

impl Monkey {
    fn inspect(&self) -> Option<u64> {
        let mut deque = self.items.take();
        let item = deque.pop_front()?;
        self.items.replace(deque);
        self.inspected.replace(self.inspected.take() + 1);
        Some(item)
    }

    fn operate(&self, item: u64) -> u64 {
        (self.operation.op)(item)
    }

    fn next(&self, item: u64) -> usize {
        self.next[(item % self.divisor == 0) as usize]
    }

    fn receive(&self, item: u64) {
        let mut deque = self.items.take();
        deque.push_back(item);
        self.items.replace(deque);
    }
}

fn puzzle1(input: &str) -> u64 {
    let mut monkeys = build(input);
    let lookup: HashMap<_, _> = monkeys.iter().map(|m| (m.id, m)).collect();
    for _ in 0..20 {
        for m in &monkeys {
            while let Some(item) = m.inspect() {
                let item = m.operate(item) / 3; // worry level
                lookup.get(&m.next(item)).unwrap().receive(item);
            }
        }
    }
    monkeys.sort_unstable_by(|m1, m2| m2.inspected.cmp(&m1.inspected));
    monkeys[0].inspected.take() * monkeys[1].inspected.take()
}

fn puzzle2(input: &str) -> u64 {
    let mut monkeys = build(input);
    let common_multiple = monkeys.iter().map(|m| m.divisor).fold(1, |acc, divisor| acc * divisor);
    let lookup: HashMap<_, _> = monkeys.iter().map(|m| (m.id, m)).collect();
    for _ in 0..10000 {
        for m in &monkeys {
            while let Some(item) = m.inspect() {
                let item = m.operate(item) % common_multiple;
                let to = m.next(item);
                lookup.get(&to).unwrap().receive(item);
            }
        }
        for m in &monkeys {
            let items = m.items.take();
            m.items.replace(items);
        }
    }
    monkeys.sort_unstable_by(|m1, m2| m2.inspected.cmp(&m1.inspected));
    monkeys[0].inspected.take() * monkeys[1].inspected.take()
}

fn build(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(|block| {
        let lines: Vec<_> = block.lines().collect();
        let id = lines[0].trim_start_matches("Monkey ").trim_end_matches(':').parse::<usize>().unwrap();
        let items: VecDeque<_> = lines[1].trim_start_matches("  Starting items: ").split(", ").flat_map(|s| s.parse::<u64>()).collect();
        let ops: Vec<_> = lines[2].trim_start_matches("  Operation: new = ").split(' ').collect();
        let operation = match (ops[0], ops[1], ops[2]) {
            ("old", operator, "old") => match operator {
                "+" => Operation { op: Box::new(|old| old + old) },
                "-" => Operation { op: Box::new(|old| old - old) },
                "*" => Operation { op: Box::new(|old| old * old) },
                "/" => Operation { op: Box::new(|old| old / old) },
                _ => unreachable!()
            }
            ("old", operator, operand) => {
                let operand = operand.parse::<u64>().unwrap();
                match operator {
                    "+" => Operation { op: Box::new(move |old| old + operand) },
                    "-" => Operation { op: Box::new(move |old| old - operand) },
                    "*" => Operation { op: Box::new(move |old| old * operand) },
                    "/" => Operation { op: Box::new(move |old| old / operand) },
                    _ => unreachable!()
                }
            }
            _ => unreachable!(),
        };
        let divisor = lines[3].trim_start_matches("  Test: divisible by ").parse::<u64>().unwrap();
        let mut throw_to = [0; 2];
        throw_to[1] = lines[4].trim_start_matches("    If true: throw to monkey ").parse::<usize>().unwrap();
        throw_to[0] = lines[5].trim_start_matches("    If false: throw to monkey ").parse::<usize>().unwrap();
        Monkey { id, items: Cell::new(items), operation, divisor, next: throw_to, inspected: Cell::new(0) }
    }).collect()
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &'static str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1(&INPUT), 10605);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2(&INPUT), 2713310158);
    }
}


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("puzzle1:{:?}", puzzle1(&input));
    println!("puzzle2:{:?}", puzzle2(&input));
}
