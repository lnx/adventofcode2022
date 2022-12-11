use std::fs;

fn puzzle1(input: &str) -> i32 {
    let mut x = 1;
    let mut cycle = 0;
    input.lines().flat_map(|line| {
        let instructions = if line == "noop" {
            vec![0]
        } else {
            vec![0, line.trim_start_matches("addx ").parse::<i32>().unwrap()]
        };
        instructions.into_iter().flat_map(|incr| {
            cycle += 1;
            let res = if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220 {
                Some(cycle * x)
            } else {
                None
            };
            x += incr;
            res
        }).collect::<Vec<_>>()
    }).sum()
}

fn puzzle2(input: &str) {
    let mut pos = 1;
    let mut crt = 0;
    let chs: Vec<_> = input.lines().flat_map(|line| {
        let instructions = if line == "noop" {
            vec![0]
        } else {
            vec![0, line.trim_start_matches("addx ").parse::<i32>().unwrap()]
        };
        instructions.into_iter().map(|incr| {
            let draw = if pos - 1 <= crt && crt <= pos + 1 { '#' } else { '.' };
            pos += incr;
            crt = (crt + 1) % 40;
            draw
        }).collect::<Vec<_>>()
    }).collect();
    for chunk in chs.chunks(40) {
        println!("{:?}", String::from_iter(chunk));
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_puzzle1() {
        let input = fs::read_to_string("demo.txt").unwrap();
        assert_eq!(puzzle1(&input), 13140);
    }
}


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("puzzle1:{:?}", puzzle1(&input));
    puzzle2(&input);
}