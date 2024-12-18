use std::str::FromStr;

use anyhow::{anyhow, Result};
use regex::Regex;

pub fn day13(input: String) -> Result<()> {
    let day_parsed_input = parse_day(input)?;
    let day_a_total = compute_day_a(&day_parsed_input)?;
    println!("Day 13 A Input result: {:?}", day_a_total);
    let day_b_total = compute_day_b(&day_parsed_input)?;
    println!("Day 13 B Input result: {:?}", day_b_total);

    Ok(())
}

struct ClawMachine {
    cost_a: i32,
    cost_b: i32,
    button_a: (i32, i32),
    button_b: (i32, i32),
    prize: (i32, i32),
}

impl FromStr for ClawMachine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut line_iter = s.lines();
        let button_a_str = line_iter.next().unwrap();
        let button_b_str = line_iter.next().unwrap();
        let prize_str = line_iter.next().unwrap();

        let button_regex = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)")?;
        let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)")?;

        let button_a_result = button_regex.captures(button_a_str).unwrap();
        let button_a = (
            button_a_result.get(1).unwrap().as_str().parse::<i32>()?,
            button_a_result.get(2).unwrap().as_str().parse::<i32>()?,
        );

        let button_b_result = button_regex.captures(button_b_str).unwrap();
        let button_b = (
            button_b_result.get(1).unwrap().as_str().parse::<i32>()?,
            button_b_result.get(2).unwrap().as_str().parse::<i32>()?,
        );

        let prize_result = prize_regex.captures(prize_str).unwrap();
        let prize = (
            prize_result.get(1).unwrap().as_str().parse::<i32>()?,
            prize_result.get(2).unwrap().as_str().parse::<i32>()?,
        );

        Ok(Self {
            cost_a: 3,
            cost_b: 1,
            button_a,
            button_b,
            prize,
        })
    }
}

impl ClawMachine {
    fn solve(&self) -> Result<(i32, i32)> {
        /*
           1: p_x = a_x * a + b_x * b
           2: p_y = a_y * a + b_y * b

           row_2_div = b_y/b_x

           3: p_y * b_x/b_y =
        */
        // let row_x = (self.prize.0, self.button_a.0, self.button_b.0);
        // let row_y = (self.prize.1, self.button_a.1, self.button_b.1);

        // println!("{} {}", row_y.2, row_x.2);
        // let row_2_div = row_y.2 / row_x.2;
        // if row_2_div == 0 {
        //     return Err(anyhow!("Would have divided by 0"));
        // }
        // let row_3 = (row_2.0 / row_2_div, row_2.1 / row_2_div, row_2.2 / row_2_div);

        // let row_4 = (row_1.0 - row_3.0, row_1.1 - row_3.1, row_1.2 - row_3.2);
        // if row_4.2 != 0 {
        //     return Err(anyhow!("Unable to simplify"));
        // }

        // let a = row_4.0 / row_4.1;
        // let b = (row_1.0 - row_1.1 * a) / row_1.2;

        todo!()
        // Ok((a, b))
    }

    fn get_cost_to_solve(&self) -> Result<i32> {
        let (a, b) = self.solve()?;

        Ok(a * self.cost_a + b * self.cost_b)
    }
}

fn parse_day(input: String) -> Result<Vec<ClawMachine>> {
    input
        .split("\n\n")
        .map(|entry| ClawMachine::from_str(entry))
        .collect()
}

fn compute_day_a(input: &Vec<ClawMachine>) -> Result<i32> {
    let costs = input
        .iter()
        .map(|machine| machine.get_cost_to_solve())
        .filter(|e| e.is_ok())
        .collect::<Result<Vec<_>>>()?;
    Ok(costs.iter().sum())
}

fn compute_day_b(input: &Vec<ClawMachine>) -> Result<u32> {
    todo!();
}
