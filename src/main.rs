use anyhow::Result;

mod file_ops;
use file_ops::{read_example_input, read_input};
mod days;
use days::{compute_day01a, compute_day01b, parse_day01};

fn main() -> Result<()> {
    let day01_example_input = read_example_input(1)?;
    let day01_parsed_example_input = parse_day01(day01_example_input)?;
    let day01a_example_total = compute_day01a(&day01_parsed_example_input)?;
    let day01b_example_total = compute_day01b(&day01_parsed_example_input)?;
    println!("Day 01 A Example input result: {:?}", day01a_example_total);
    println!("Day 01 B Example input result: {:?}", day01b_example_total);

    let day01_input = read_input(1)?;
    let day01_parsed_input = parse_day01(day01_input)?;
    let day01a_total = compute_day01a(&day01_parsed_input)?;
    let day01b_total = compute_day01b(&day01_parsed_input)?;
    println!("Day 01 A Input result: {:?}", day01a_total);
    println!("Day 01 B Input result: {:?}", day01b_total);

    Ok(())
}
