use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Result;

mod file_ops;
use clap::Parser;
use days::day01;
use days::day02;
use days::day03;
use days::day04;
use days::day05;
use days::day06;
use days::day07;
use days::day08;
use days::day09;
use days::day10;
use days::day11;
use days::day12;
use file_ops::{read_example_input, read_input};
mod days;

#[derive(Debug, Clone)]
enum InputType {
    MAIN,
    EXAMPLE,
}

impl FromStr for InputType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "main" => Ok(Self::MAIN),
            "example" => Ok(Self::EXAMPLE),
            x => Err(anyhow!("Unable to construct InputType from {x}")),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input_type: InputType,

    #[arg(short, long)]
    day: i32,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let day = args.day;
    let input_type = args.input_type;

    let day_input = match input_type {
        InputType::EXAMPLE => read_example_input(day),
        InputType::MAIN => read_input(day),
    }?;

    match day {
        01 => day01(day_input),
        02 => day02(day_input),
        03 => day03(day_input),
        04 => day04(day_input),
        05 => day05(day_input),
        06 => day06(day_input),
        07 => day07(day_input),
        08 => day08(day_input),
        09 => day09(day_input),
        10 => day10(day_input),
        11 => day11(day_input),
        12 => day12(day_input),
        _ => Err(anyhow!("Day not found!")),
    }
}
