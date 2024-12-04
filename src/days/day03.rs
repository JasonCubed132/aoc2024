use anyhow::Result;
use regex::Regex;

use super::list_ops::count_items_in_list;

#[derive(Debug)]
enum INSTRUCTIONS {
    MUL(i32, i32),
    ENABLE,
    DISABLE,
}

pub fn day03(input: String) -> Result<()> {
    let day_parsed_input = parse_day(input)?;
    let day_a_total = compute_day_a(&day_parsed_input)?;
    println!("Day 03 A Input result: {:?}", day_a_total);
    let day_b_total = compute_day_b(&day_parsed_input)?;
    println!("Day 03 B Input result: {:?}", day_b_total);

    Ok(())
}

fn parse_day(input: String) -> Result<Vec<INSTRUCTIONS>> {
    let re = Regex::new(
        r"(?<mul>mul[(](?<mul_left>\d+),(?<mul_right>\d+)[)])|(?<do>do[(][)])|(?<dont>don't[(][)])",
    )?;
    let mut instructions = Vec::new();
    for result in re.captures_iter(&input) {
        match result.name("mul") {
            Some(_) => {
                let left: i32 = result.name("mul_left").unwrap().as_str().parse()?;
                let right: i32 = result.name("mul_right").unwrap().as_str().parse()?;
                instructions.push(INSTRUCTIONS::MUL(left, right));
            }
            None => {}
        }
        match result.name("do") {
            Some(_) => {
                instructions.push(INSTRUCTIONS::ENABLE);
            }
            _ => {}
        }
        match result.name("dont") {
            Some(_) => {
                instructions.push(INSTRUCTIONS::DISABLE);
            }
            _ => {}
        }
    }

    Ok(instructions)
}

fn compute_day_a(input: &Vec<INSTRUCTIONS>) -> Result<i32> {
    Ok(input.into_iter().fold(0, |acc, inst| match inst {
        INSTRUCTIONS::MUL(x, y) => acc + x * y,
        _ => acc,
    }))
}

fn compute_day_b(input: &Vec<INSTRUCTIONS>) -> Result<i32> {
    let (total, _) = input
        .into_iter()
        .fold((0, true), |(acc, state), inst| match inst {
            INSTRUCTIONS::MUL(x, y) => {
                if state {
                    (acc + x * y, state)
                } else {
                    (acc, state)
                }
            }
            INSTRUCTIONS::ENABLE => (acc, true),
            INSTRUCTIONS::DISABLE => (acc, false),
        });

    Ok(total)
}
