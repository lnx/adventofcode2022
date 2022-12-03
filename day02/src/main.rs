use std::fs;

#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

struct Round {
    opponent: Shape,
    you: Shape,
}

// https://rust-lang.github.io/rfcs/2005-match-ergonomics.html

fn score1(input: &String) -> u32 {
    let rounds: Vec<_> = input.lines().map(|line| {
        let shapes: Vec<_> = line.chars().filter(|&c| c != ' ').collect();
        let opponent = match shapes[0] {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => unreachable!(),
        };
        let you = match shapes[1] {
            'X' => Shape::Rock,
            'Y' => Shape::Paper,
            'Z' => Shape::Scissors,
            _ => unreachable!(),
        };
        Round { opponent, you }
    }).collect();
    rounds.iter().map(|r| score_single_round(r.opponent, r.you)).sum()
}

fn score2(input: &String) -> u32 {
    let rounds: Vec<_> = input.lines().map(|line| {
        let shapes: Vec<_> = line.chars().filter(|&c| c != ' ').collect();
        let opponent = match shapes[0] {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => unreachable!(),
        };
        let you = match shapes[1] {
            'X' => { // lose
                match opponent {
                    Shape::Rock => Shape::Scissors,
                    Shape::Paper => Shape::Rock,
                    Shape::Scissors => Shape::Paper,
                }
            }
            'Y' => { // draw
                opponent
            }
            'Z' => { // win
                match opponent {
                    Shape::Rock => Shape::Paper,
                    Shape::Paper => Shape::Scissors,
                    Shape::Scissors => Shape::Rock,
                }
            }
            _ => unreachable!(),
        };
        Round { opponent, you }
    }).collect();
    rounds.iter().map(|r| score_single_round(r.opponent, r.you)).sum()
}

fn score_single_round(opponent: Shape, you: Shape) -> u32 {
    match (opponent, you) {
        (Shape::Rock, Shape::Paper) | (Shape::Paper, Shape::Scissors) | (Shape::Scissors, Shape::Rock) => you as u32 + 6,
        (Shape::Rock, Shape::Rock) | (Shape::Paper, Shape::Paper) | (Shape::Scissors, Shape::Scissors) => you as u32 + 3,
        (Shape::Rock, Shape::Scissors) | (Shape::Paper, Shape::Rock) | (Shape::Scissors, Shape::Paper) => you as u32 + 0,
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("score1:{:?}", score1(&input));
    println!("score2:{:?}", score2(&input));
}
