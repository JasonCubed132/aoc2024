use std::{collections::HashSet, str::FromStr};

use anyhow::{anyhow, Result};

use crate::days::grid_ops::{Cell, Coord};

use super::grid_ops::Grid;

pub fn day06(input: String) -> Result<()> {
    let day_parsed_input = parse_day(input)?;
    let day_a_total = compute_day_a(&day_parsed_input)?;
    println!("Day 06 A Input result: {:?}", day_a_total);
    let day_b_total = compute_day_b(&day_parsed_input)?;
    println!("Day 06 B Input result: {:?}", day_b_total);

    Ok(())
}

#[derive(Clone, PartialEq, Debug)]
enum SpaceContents {
    EMPTY,
    OBSTACLE,
    GUARD,
}

#[derive(Debug)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    pub fn get_delta(&self) -> (i32, i32) {
        match self {
            Self::UP => (0, -1),
            Self::RIGHT => (1, 0),
            Self::DOWN => (0, 1),
            Self::LEFT => (-1, 0),
        }
    }
    pub fn turn_right(self) -> Self {
        match self {
            Self::UP => Self::RIGHT,
            Self::RIGHT => Self::DOWN,
            Self::DOWN => Self::LEFT,
            Self::LEFT => Self::UP,
        }
    }
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

fn compute_day_a(input: &Grid<SpaceContents>) -> Result<usize> {
    let mut grid = input.clone();
    let mut direction = Direction::UP;
    let mut coord = grid.find_first(SpaceContents::GUARD)?.unwrap();
    grid.set_cell_contents(&coord, SpaceContents::EMPTY)?;

    let mut visited_coords: HashSet<Coord> = HashSet::new();
    visited_coords.insert(coord.clone());
    loop {
        let dir = direction.get_delta();
        let projection: Vec<(Coord, SpaceContents)> = grid
            .get_projection_iter(coord.clone(), dir.0, dir.1)
            .skip(1)
            .collect();
        let projection_len = projection.clone().len();

        let path: Vec<Coord> = projection
            .iter()
            .take_while(|item| item.1 == SpaceContents::EMPTY)
            .map(|item| item.0.clone())
            .map(|item| {
                visited_coords.insert(item.clone());
                item
            })
            .collect();
        let path_len = path.clone().len();

        if path_len == projection_len {
            break;
        }

        match path.last().cloned() {
            Some(new) => {
                coord = new;
            }
            None => {}
        }
        direction = direction.turn_right();
    }
    Ok(visited_coords.len())
}

fn compute_day_b(input: &Grid<SpaceContents>) -> Result<i32> {
    todo!();
}
