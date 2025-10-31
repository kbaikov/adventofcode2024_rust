use std::fs;

use anyhow;
mod part1;
mod part2;

fn main() -> anyhow::Result<()> {
    let result = part1::process(&fs::read_to_string("day02_input.txt").unwrap())?;
    println!("Part 1 result: {}", result);

    let result = part2::process(&fs::read_to_string("day02_input.txt").unwrap())?;
    println!("Part 2 result: {}", result);

    Ok(())
}
