use anyhow::anyhow;
use anyhow::Result;

mod file_ops;
use file_ops::{read_example_input, read_input};
mod days;
use days::day01;
use days::day02;
use days::day03;
use days::day04;

enum InputType {
    MAIN,
    EXAMPLE,
}

fn main() -> Result<()> {
    let day = 4;
    let input_type = InputType::EXAMPLE;

    let day_input = match input_type {
        InputType::EXAMPLE => read_example_input(day),
        InputType::MAIN => read_input(day),
    }?;

    match day {
        1 => day01(day_input),
        2 => day02(day_input),
        3 => day03(day_input),
        4 => day04(day_input),
        _ => Err(anyhow!("Day not found!")),
    }
}
