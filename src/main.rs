use anyhow::Result;

mod file_ops;
use file_ops::{read_example_input, read_input};
mod days;
use days::day01;

enum InputType {
    MAIN,
    EXAMPLE,
}

fn main() -> Result<()> {
    let day = 1;
    let input_type = InputType::MAIN;

    match day {
        1 => match input_type {
            InputType::EXAMPLE => {
                let day01_example_input = read_example_input(1)?;
                day01(day01_example_input)?;
            }
            InputType::MAIN => {
                let day01_input = read_input(1)?;
                day01(day01_input)?;
            }
        },
        _ => {
            println!("Day not found!")
        }
    }

    Ok(())
}
