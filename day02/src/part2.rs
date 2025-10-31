/// influenced by https://github.com/ChristopherBiscardi/advent-of-code/
use anyhow::anyhow;
use nom::Parser;
use nom::{
    IResult,
    character::complete::{line_ending, space1},
    multi::separated_list1,
};

use nom::character::complete;

use std::iter::zip;

pub fn process(input: &str) -> anyhow::Result<String> {
    let (_, reports) = parse(input).map_err(|e| anyhow!("Parsing error: {}", e))?;

    Ok(reports
        .iter()
        .filter(|report| {
            if !is_safe(report) {
                for index in 0..report.len() {
                    let mut new_report = (*report).clone();
                    new_report.remove(index);
                    if is_safe(&new_report) {
                        return true;
                    } else {
                        continue;
                    }
                }
                return false;
            } else {
                true
            }
        })
        .count()
        .to_string())
}

type Report = Vec<i32>;

fn parse(input: &str) -> IResult<&str, Vec<Report>> {
    separated_list1(line_ending, separated_list1(space1, complete::i32)).parse(input)
}

fn is_safe(report: &Report) -> bool {
    let d =
        zip(report.iter(), report.iter().skip(1)).all(|(l, r)| (1..=3).contains(&l.abs_diff(*r)));
    if !d {
        return false;
    }
    let slope1 = zip(report.iter(), report.iter().skip(1)).all(|(l, r)| (l - r) > 0);
    let slope2 = zip(report.iter(), report.iter().skip(1)).all(|(l, r)| (l - r) < 0);

    d && (slope1 || slope2)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::process;

    #[test]
    fn test_example_data() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("4", process(input).unwrap());
    }

    #[test]
    fn test_real_data() {
        let input = fs::read_to_string("day02_input.txt").unwrap();
        assert_eq!("426", process(&input).unwrap());
    }
}
