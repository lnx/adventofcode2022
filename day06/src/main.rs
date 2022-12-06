use std::collections::HashSet;
use std::fs;

fn puzzle(input: &str, window: usize) -> usize {
    for (i, chs) in input.chars().collect::<Vec<_>>().windows(window).enumerate() {
        if chs.iter().copied().collect::<HashSet<_>>().len() == window {
            return i + window;
        }
    }
    input.len()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(puzzle("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(puzzle("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(puzzle("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(puzzle("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(puzzle("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(puzzle("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(puzzle("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("puzzle1:{:?}", puzzle(&input, 4));
    println!("puzzle2:{:?}", puzzle(&input, 14));
}
