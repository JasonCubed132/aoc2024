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
    let mut frequencies_present = input.get_all_elements_present();
    frequencies_present.retain(|c| c != &'.');

    let mut all_antinodes = HashSet::new();

    for frequency in frequencies_present {
        let coords = input.get_all_coords_matching(&frequency);
        let mut anti_nodes = HashSet::new();

        for (i, coord_1) in coords.iter().enumerate() {
            for (_, coord_2) in coords.iter().enumerate().skip(i + 1) {
                let delta = *coord_2 - *coord_1;

                let coord_a = coord_1.add_delta(&delta.get_neg());
                let coord_b = coord_2.add_delta(&delta);

                let _ = coord_a.and_then(|inner| {
                    anti_nodes.insert(inner);
                    Ok(())
                });
                let _ = coord_b.and_then(|inner| {
                    anti_nodes.insert(inner);
                    Ok(())
                });
            }
        }

        for antinode in anti_nodes {
            all_antinodes.insert(antinode);
        }
    }

    Ok(all_antinodes.len())
}

fn compute_day_b(input: &Grid<char>) -> Result<usize> {
    let mut frequencies_present = input.get_all_elements_present();
    frequencies_present.retain(|c| c != &'.');

    let mut all_antinodes = HashSet::new();

    for frequency in frequencies_present {
        let coords = input.get_all_coords_matching(&frequency);
        let mut anti_nodes = HashSet::new();

        for (i, &coord_1) in coords.iter().enumerate() {
            for (_, &coord_2) in coords.iter().enumerate().skip(i + 1) {
                anti_nodes.insert(coord_1);
                anti_nodes.insert(coord_2);

                let delta = coord_2 - coord_1;

                let mut coord_a_result = coord_1.add_delta(&delta.get_neg());
                while !coord_a_result.is_err() {
                    let coord_a = coord_a_result.unwrap();
                    anti_nodes.insert(coord_a);

                    coord_a_result = coord_a.add_delta(&delta.get_neg());
                }

                let mut coord_b_result = coord_2.add_delta(&delta);
                while !coord_b_result.is_err() {
                    let coord_b = coord_b_result.unwrap();
                    anti_nodes.insert(coord_b);

                    coord_b_result = coord_b.add_delta(&delta);
                }
            }
        }

        for antinode in anti_nodes {
            all_antinodes.insert(antinode);
        }
    }

    Ok(all_antinodes.len())
}
