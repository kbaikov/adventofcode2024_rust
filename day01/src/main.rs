use std::fs;

use anyhow;
mod part1;
mod part2;

fn main() -> anyhow::Result<()> {
    let result = part1::process(&fs::read_to_string("day01_input.txt").unwrap())?;
    println!("Part 1 resutl: {}", result);

    let result = part2::process(&fs::read_to_string("day01_input.txt").unwrap())?;
    println!("Part 2 resutl: {}", result);

    Ok(())
}
