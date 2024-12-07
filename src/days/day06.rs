use std::str::FromStr;

use anyhow::{anyhow, Result};

use crate::days::grid_ops::Cell;

use super::grid_ops::Grid;

pub fn day06(input: String) -> Result<()> {
    let day_parsed_input = parse_day(input)?;
    let day_a_total = compute_day_a(&day_parsed_input)?;
    println!("Day 01 A Input result: {:?}", day_a_total);
    let day_b_total = compute_day_b(&day_parsed_input)?;
    println!("Day 01 B Input result: {:?}", day_b_total);

    Ok(())
}

#[derive(Clone)]
enum SpaceContents {
    EMPTY,
    OBSTACLE,
    GUARD,
}

impl FromStr for SpaceContents {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "." => Ok(Self::EMPTY),
            "#" => Ok(Self::OBSTACLE),
            "^" => Ok(Self::GUARD),
            x => Err(anyhow!("Cannot make SpaceContents from {:?}", x)),
        }
    }
}

fn make_cell_from_string(input: String) -> Result<Cell<SpaceContents>> {
    let res = SpaceContents::from_str(&input)?;
    Ok(Cell::new(res))
}

fn make_grid_from_string(input: String) -> Result<Grid<SpaceContents>> {
    let res = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| make_cell_from_string(char.to_string()))
                .collect::<Result<Vec<Cell<SpaceContents>>>>()
        })
        .collect::<Result<Vec<Vec<Cell<SpaceContents>>>>>()?;
    Ok(Grid::new(res))
}

fn parse_day(input: String) -> Result<Grid<SpaceContents>> {
    make_grid_from_string(input)
}

fn compute_day_a(input: &Grid<SpaceContents>) -> Result<i32> {
    todo!();
}

fn compute_day_b(input: &Grid<SpaceContents>) -> Result<i32> {
    todo!();
}
