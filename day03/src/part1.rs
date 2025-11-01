/// influenced by https://github.com/ChristopherBiscardi/advent-of-code/
use anyhow;
use regex::Regex;

pub fn process(input: &str) -> anyhow::Result<String> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut results = vec![];
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        results.push((a.parse::<u64>()?, b.parse::<u64>()?));
    }
    let s: u64 = results.iter().map(|(a, b)| a * b).sum();

    Ok(s.to_string())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::process;

    #[test]
    fn test_example_data() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input).unwrap());
    }

    #[test]
    fn test_real_data() {
        let input = fs::read_to_string("input.txt").unwrap();
        assert_eq!("184576302", process(&input).unwrap());
    }
}
