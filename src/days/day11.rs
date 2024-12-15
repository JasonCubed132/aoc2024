use std::{collections::HashMap, fmt::Display, str::FromStr};

use anyhow::Result;

use super::list_ops::count_items_in_list;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
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

impl Display for Stone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.num)
    }
}

impl Stone {
    fn advance_state(&mut self) -> Result<Option<Self>> {
        if self.num == 0 {
            self.num = 1;
            Ok(None)
        } else if self.num.to_string().len() % 2 == 0 {
            let num_str = self.num.to_string();
            let num_str_len = num_str.len();
            let (left, right) = num_str.split_at(num_str_len / 2);

            self.num = left.parse::<u64>()?;
            let new_stone = Stone::from_str(right)?;
            Ok(Some(new_stone))
        } else {
            self.num *= 2024;
            Ok(None)
        }
    }
}

#[derive(Clone, Debug)]
struct PlutonianPebbles {
    counts: HashMap<Stone, u64>,
}

impl FromStr for PlutonianPebbles {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let stones = s
            .split(' ')
            .map(|num| Stone::from_str(num))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self::new(stones))
    }
}

impl Display for PlutonianPebbles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stones = self
            .counts
            .iter()
            .map(|(stone, count)| format!("({}: {}", stone, count))
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "{}", stones)
    }
}

impl PlutonianPebbles {
    fn new(stones: Vec<Stone>) -> Self {
        let counts = count_items_in_list(&stones);
        let mut new_counts = HashMap::new();
        for (&stone, count) in counts.into_iter() {
            new_counts.insert(stone, count as u64);
        }
        Self { counts: new_counts }
    }

    fn advance_state(&mut self) -> Result<()> {
        let mut new_counts = HashMap::new();
        for (stone, count) in &self.counts {
            let mut cloned_stone = stone.clone();

            match cloned_stone.advance_state()? {
                Some(new_stone) => {
                    new_counts
                        .entry(new_stone)
                        .and_modify(|current: &mut u64| *current += *count)
                        .or_insert(*count);
                }
                None => {}
            }

            new_counts
                .entry(cloned_stone)
                .and_modify(|current| *current += count)
                .or_insert(*count);
        }

        self.counts = new_counts;

        Ok(())
    }

    fn get_num_stones(&self) -> u64 {
        let mut total = 0;
        for (_, count) in &self.counts {
            total += count;
        }

        total as u64
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

fn compute_day_a(input: &PlutonianPebbles) -> Result<u64> {
    let mut current_state = input.clone();
    for i in 0..25 {
        current_state.advance_state()?;
        println!("Iteration {i}");
    }

    Ok(current_state.get_num_stones())
}

fn compute_day_b(input: &PlutonianPebbles) -> Result<u64> {
    let mut current_state = input.clone();
    for i in 0..75 {
        current_state.advance_state()?;
        println!("Iteration {i}");
    }

    Ok(current_state.get_num_stones())
}
