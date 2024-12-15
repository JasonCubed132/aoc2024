use std::collections::HashSet;

use anyhow::Result;

use crate::days::grid_ops::Coord;

use super::grid_ops::{Cell, Grid};

pub fn day08(input: String) -> Result<()> {
    let day_parsed_input = parse_day(input)?;
    let day_a_total = compute_day_a(&day_parsed_input)?;
    println!("Day 08 A Input result: {:?}", day_a_total);
    let day_b_total = compute_day_b(&day_parsed_input)?;
    println!("Day 08 B Input result: {:?}", day_b_total);

    Ok(())
}

fn parse_day(input: String) -> Result<Grid<char>> {
    Ok(Grid::new(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| Cell::new(c))
                    .collect::<Vec<Cell<char>>>()
            })
            .collect::<Vec<Vec<Cell<char>>>>(),
    )?)
}

fn compute_day_a(input: &Grid<char>) -> Result<usize> {
    let frequencies_present =
        input
            .get_grid()
            .iter()
            .flatten()
            .fold(HashSet::new(), |mut acc, item| {
                if **item != '.' {
                    acc.insert(**item);
                }
                acc
            });

    let mut all_antinodes = HashSet::new();

    for frequency in frequencies_present {
        let mut coords = Vec::new();
        for (y, row) in input.get_grid().iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if **cell == frequency {
                    coords.push(Coord::new(x, y, input.get_num_cols(), input.get_num_rows()))
                }
            }
        }

        println!(
            "Frequency {:?} Coords {}",
            frequency,
            coords
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );

        let mut anti_nodes = HashSet::new();

        for i in 0..coords.len() {
            let coord_1 = &coords[i];
            for j in i + 1..coords.len() {
                let coord_2 = &coords[j];

                let delta = *coord_2 - *coord_1;

                let coord_a = coord_1.add_delta(&delta.get_neg());
                let coord_b = coord_2.add_delta(&delta);

                println!(
                    "{} {} {} {} {} {}",
                    i, j, coord_a, coord_1, coord_2, coord_b
                );

                match coord_a {
                    Ok(coord) => {
                        anti_nodes.insert(coord);
                    }
                    Err(_) => {}
                }
                match coord_b {
                    Ok(coord) => {
                        anti_nodes.insert(coord);
                    }
                    Err(_) => {}
                }
            }
        }

        for antinode in anti_nodes {
            all_antinodes.insert(antinode);
        }
    }

    Ok(all_antinodes.len())
}

fn compute_day_b(_input: &Grid<char>) -> Result<u32> {
    todo!();
}
