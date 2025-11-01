/// influenced by https://github.com/ChristopherBiscardi/advent-of-code/
use anyhow;

use regex::Regex;

pub fn process(input: &str) -> anyhow::Result<String> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut results = vec![];
    let mut enabled = true;

    for m in re.captures_iter(input) {
        let instruction = m.get(0).unwrap().as_str();
        if instruction == "do()" {
            enabled = true;
        } else if instruction == "don't()" {
            enabled = false;
        } else if enabled {
            results.push((
                m.get(1).unwrap().as_str().parse::<u64>()?,
                m.get(2).unwrap().as_str().parse::<u64>()?,
            ));
        }
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
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input).unwrap());
    }

    #[test]
    fn test_real_data() {
        let input = fs::read_to_string("input.txt").unwrap();
        assert_eq!("118173507", process(&input).unwrap());
    }
}
