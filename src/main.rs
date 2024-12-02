use anyhow::Result;

mod file_ops;
use file_ops::{read_example_input, read_input};
mod days;
use days::day01;
use days::day02;

enum InputType {
    MAIN,
    EXAMPLE,
}

fn main() -> Result<()> {
    let day = 2;
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
        2 => match input_type {
            InputType::EXAMPLE => {
                let day02_example_input = read_example_input(2)?;
                day02(day02_example_input)?;
            }
            InputType::MAIN => {
                let day02_input = read_input(2)?;
                day02(day02_input)?;
            }
        },
        _ => {
            println!("Day not found!")
        }
    }

    Ok(())
}
