use std::str::FromStr;

use anyhow::Result;

#[derive(Clone)]
struct Stone {
    num: u64,
}

impl FromStr for Stone {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        let num = s.parse::<u64>()?;

        Ok(Self { num })
    }
}

impl Stone {
    fn new(num: u64) -> Self {
        Self { num }
    }

    fn advance_state(self) -> Result<Vec<Self>> {
        if self.num == 0 {
            let mut result = Vec::new();
            result.push(Stone::new(1));
            Ok(result)
        } else if self.num.to_string().len() % 2 == 0 {
            let num_str = self.num.to_string();
            let num_str_len = num_str.len();
            let (left, right) = num_str.split_at(num_str_len / 2);

            let mut result = Vec::new();
            result.push(Stone::from_str(left)?);
            result.push(Stone::from_str(right)?);
            Ok(result)
        } else {
            let mut result = Vec::new();
            result.push(Stone::new(self.num * 2024));
            Ok(result)
        }
    }
}

#[derive(Clone)]
struct PlutonianPebbles {
    stones: Vec<Stone>,
}

impl FromStr for PlutonianPebbles {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let stones = s
            .split(' ')
            .map(|num| Stone::from_str(num))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { stones })
    }
}

impl PlutonianPebbles {
    fn advance_state(self) -> Result<Self> {
        let mut new_stones = Vec::new();

        for stone in self.stones {
            let mut result = stone.advance_state()?;
            new_stones.append(&mut result);
        }

        Ok(Self { stones: new_stones })
    }

    fn get_num_stones(self) -> usize {
        self.stones.len()
    }
}

pub fn day11(input: String) -> Result<()> {
    let day_parsed_input = parse_day(input)?;
    let day_a_total = compute_day_a(&day_parsed_input)?;
    println!("Day 11 A Input result: {:?}", day_a_total);
    let day_b_total = compute_day_b(&day_parsed_input)?;
    println!("Day 11 B Input result: {:?}", day_b_total);

    Ok(())
}

fn parse_day(input: String) -> Result<PlutonianPebbles> {
    PlutonianPebbles::from_str(&input)
}

fn compute_day_a(input: &PlutonianPebbles) -> Result<usize> {
    let mut current_state = input.clone();
    for _ in 0..25 {
        current_state = current_state.advance_state()?;
    }

    Ok(current_state.get_num_stones())
}

fn compute_day_b(input: &PlutonianPebbles) -> Result<u64> {
    todo!();
}
