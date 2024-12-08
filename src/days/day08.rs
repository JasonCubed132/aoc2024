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
    let used_cells = input
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

    for frequency in used_cells {
        let mut coords = Vec::new();
        for (y, row) in input.get_grid().iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if **cell == frequency {
                    coords.push(Coord::new(x, y, input.get_num_cols(), input.get_num_rows()))
                }
            }
        }

        println!("Frequency {:?} Coords {:?}", frequency, coords);

        let mut anti_nodes = HashSet::new();

        for i in 0..coords.len() {
            let coord_1 = &coords[i];
            for j in i..coords.len() {
                let coord_2 = &coords[j];

                let delta = coord_1.get_delta(coord_2);

                let coord_a = coord_1.clone() + delta.clone();
                let coord_b = coord_2.clone() - delta.clone();

                println!("{:?} {:?}", coord_a, coord_b);

                anti_nodes.insert(coord_a);
                anti_nodes.insert(coord_b);
            }
        }

        for antinode in anti_nodes {
            all_antinodes.insert(antinode);
        }
    }

    Ok(all_antinodes.len())
}

fn compute_day_b(input: &Grid<char>) -> Result<u32> {
    todo!();
}
