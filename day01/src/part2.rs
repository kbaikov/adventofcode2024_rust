/// influenced by https://github.com/ChristopherBiscardi/advent-of-code/blob/main/2024/rust/day-01/src/part1.rs
use anyhow;

pub fn process(input: &str) -> anyhow::Result<String> {
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<isize>().unwrap());
        right.push(items.next().unwrap().parse::<isize>().unwrap());
    }
    left.sort();
    right.sort();

    let result: usize = left
        .iter()
        .map(|n| *n as usize * right.iter().filter(|r| n == *r).count())
        .sum();
    dbg!(result);
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::process;

    #[test]
    fn test_example_data() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input).unwrap());
    }

    #[test]
    fn test_real_data() {
        let input = fs::read_to_string("day01_input.txt").unwrap();
        assert_eq!("24869388", process(&input).unwrap());
    }
}
