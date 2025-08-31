use std::fs;

use anyhow;
use day02::part1;
use day02::part2;

#[tracing::instrument]
fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let result = part1::process(&fs::read_to_string("day02/day02_input.txt").unwrap())?;
    println!("Part 1 result: {}", result);

    let result = part2::process(&fs::read_to_string("day02/day02_input.txt").unwrap())?;
    println!("Part 2 result: {}", result);

    Ok(())
}
