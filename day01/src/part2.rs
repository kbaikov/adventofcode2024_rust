/// influenced by https://github.com/ChristopherBiscardi/advent-of-code/blob/main/2024/rust/day-01/src/part1.rs
use anyhow::Error;
use std::{fs, iter::zip};

fn main() {
    let _ = process(&fs::read_to_string("day01_input.txt").unwrap());
}

fn process(input: &str) -> Result<String, Error> {
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<isize>().unwrap());
        right.push(items.next().unwrap().parse::<isize>().unwrap());
    }
    left.sort();
    right.sort();

    let result: usize = zip(left, right).map(|(l, r)| l.abs_diff(r)).sum();
    dbg!(result);
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input).unwrap());
    }
}
