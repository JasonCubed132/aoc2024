use std::collections::HashSet;

use anyhow::Result;

use crate::days::grid_ops::Cell;

use super::grid_ops::{Coord, Delta, Grid};

pub fn day10(input: String) -> Result<()> {
    let day_parsed_input = parse_day(input)?;
    let day_a_total = compute_day_a(&day_parsed_input)?;
    println!("Day 10 A Input result: {:?}", day_a_total);
    let day_b_total = compute_day_b(&day_parsed_input)?;
    println!("Day 10 B Input result: {:?}", day_b_total);

    Ok(())
}

fn parse_day(input: String) -> Result<Grid<u32>> {
    let inner = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| Cell::new(char.to_digit(10).unwrap()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();
    Grid::new(inner)
}

fn spawn_searcher(grid: &Grid<u32>, coord: Coord, target_value: u32) -> Result<Vec<Coord>> {
    let current_value = grid.get_cell_contents(&coord)?;

    if current_value == target_value {
        let mut result = Vec::new();
        result.push(coord);
        return Ok(result);
    }

    let new_value = current_value + 1;

    let deltas = [
        Delta::new(1, 0),
        Delta::new(0, 1),
        Delta::new(-1, 0),
        Delta::new(0, -1),
    ];

    let mut all_nine_coords = Vec::new();

    for delta in deltas {
        match coord.add_delta(&delta) {
            Ok(new_coord) => {
                if grid.get_cell_contents(&new_coord)? != new_value {
                    continue;
                }

                // println!("Got {} {}", new_coord, new_value);

                let node_nine_coords = spawn_searcher(grid, new_coord, target_value)?;
                for nine_coord in node_nine_coords {
                    all_nine_coords.push(nine_coord);
                }
            }
            Err(_) => continue,
        };
    }

    // println!("{} result {}", coord, total);

    Ok(all_nine_coords)
}

fn compute_day_a(input: &Grid<u32>) -> Result<usize> {
    let zeroes = input.find_all(0)?;
    let mut total = 0;

    for zero in zeroes {
        // println!("Zero detected at {:?}", zero);
        let node_nine_coords: Vec<Coord> = spawn_searcher(input, zero, 9)?;

        let without_duplicates: HashSet<Coord> = HashSet::from_iter(node_nine_coords.into_iter());
        // println!("Result {}", without_duplicates.len());

        total += without_duplicates.len();
    }

    Ok(total)
}

fn compute_day_b(input: &Grid<u32>) -> Result<usize> {
    let zeroes = input.find_all(0)?;
    let mut total = 0;

    for zero in zeroes {
        // println!("Zero detected at {:?}", zero);
        let node_nine_coords: Vec<Coord> = spawn_searcher(input, zero, 9)?;

        // println!("Result {}", node_nine_coords.len());

        total += node_nine_coords.len();
    }

    Ok(total)
}
