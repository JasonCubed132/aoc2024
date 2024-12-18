use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::Result;

use crate::days::grid_ops::Delta;

use super::grid_ops::{Cell, Coord, Grid};

pub fn day12(input: String) -> Result<()> {
    let day_parsed_input = parse_day(input)?;
    let day_a_total = compute_day_a(&day_parsed_input)?;
    println!("Day 12 A Input result: {:?}", day_a_total);
    let day_b_total = compute_day_b(&day_parsed_input)?;
    println!("Day 12 B Input result: {:?}", day_b_total);

    Ok(())
}

struct GardenGroups {
    garden: Grid<char>,
}

impl FromStr for GardenGroups {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let garden = s
            .lines()
            .map(|line| line.chars().map(|char| Cell::new(char)).collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>();
        let garden_grid = Grid::new(garden)?;
        Ok(Self {
            garden: garden_grid,
        })
    }
}

impl GardenGroups {
    fn calculate_areas_and_perimetres(&self) -> Result<Vec<(u32, u32)>> {
        let mut output = Vec::new();

        let offsets = [
            Delta::new(0, 1),
            Delta::new(-1, 0),
            Delta::new(0, -1),
            Delta::new(1, 0),
        ];

        let start = Coord::new(0, 0, self.garden.get_num_cols(), self.garden.get_num_rows());
        let mut globally_explored_coords = HashSet::new();
        globally_explored_coords.insert(start);

        let mut new_region_coords_to_search = Vec::new();
        new_region_coords_to_search.push(start);

        while new_region_coords_to_search.len() > 0 {
            // Start exploration of new region based on cell we
            // know is within that region.
            let new_region_start_coord = new_region_coords_to_search.pop().unwrap();

            let mut coords_in_region = HashSet::new();
            coords_in_region.insert(new_region_start_coord);

            let mut coords_in_region_to_search = Vec::new();
            coords_in_region_to_search.push(new_region_start_coord);

            let mut fences: u32 = 0;

            // Discover new cells within region by checking neighbours.
            while coords_in_region_to_search.len() > 0 {
                let explore_anchor_coord = coords_in_region_to_search.pop().unwrap();
                let explore_anchor_cell = self.garden.get_cell_contents(&explore_anchor_coord)?;

                for offset in offsets {
                    let coord_being_explored_result = explore_anchor_coord.add_delta(&offset);

                    if coord_being_explored_result.is_err() {
                        fences += 1;
                        continue;
                    }

                    let coord_being_explored = coord_being_explored_result.unwrap();

                    let cell_being_explored =
                        self.garden.get_cell_contents(&coord_being_explored)?;

                    // If the neighbour cell's plant equals the current cell's plant,
                    // it is part of the same region and add it to the queue to be
                    // searched as part of the region. Otherwise, add it to the global
                    // search queue.
                    if cell_being_explored == explore_anchor_cell {
                        if !coords_in_region.contains(&coord_being_explored) {
                            coords_in_region_to_search.push(coord_being_explored);
                        }
                        coords_in_region.insert(coord_being_explored);
                        globally_explored_coords.insert(coord_being_explored);
                    } else {
                        fences += 1;
                        if !globally_explored_coords.contains(&coord_being_explored) {
                            new_region_coords_to_search.push(coord_being_explored);
                        }
                    }
                }
            }

            let area = coords_in_region.len() as u32;
            output.push((area, fences));

            println!(
                "Plant {} Area {} Perimeter {}",
                self.garden.get_cell_contents(&new_region_start_coord)?,
                area,
                fences
            );
        }

        Ok(output)
    }
}

fn parse_day(input: String) -> Result<GardenGroups> {
    GardenGroups::from_str(&input)
}

fn compute_day_a(input: &GardenGroups) -> Result<u32> {
    let total: u32 = input
        .calculate_areas_and_perimetres()?
        .iter()
        .map(|(area, perimeter)| area * perimeter)
        .sum();
    Ok(total)
}

fn compute_day_b(input: &GardenGroups) -> Result<u32> {
    todo!();
}
