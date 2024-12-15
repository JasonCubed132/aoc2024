use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::Result;

use crate::days::grid_ops::Delta;

use super::grid_ops::{Cell, Grid};

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
        let mut areas: HashMap<char, u32> = HashMap::new();
        let mut perimeters: HashMap<char, u32> = HashMap::new();

        let coords = self.garden.get_all_coords();

        let offsets = [
            Delta::new(0, 1),
            Delta::new(-1, 0),
            Delta::new(0, -1),
            Delta::new(1, 0),
        ];

        // TODO - each region of the same plant needs to be handled
        //        separately. maybe implement a worker
        for coord in coords {
            let plant = self.garden.get_cell_contents(&coord)?;
            areas
                .entry(plant)
                .and_modify(|count| *count += 1)
                .or_insert(1);

            let mut fences = 0;

            for offset in offsets {
                let new_coord = coord.add_delta(&offset);

                if new_coord.is_err() {
                    fences += 1;
                    continue;
                }

                let neighbour_cell = self.garden.get_cell_contents(&new_coord.unwrap())?;

                if neighbour_cell != plant {
                    fences += 1;
                }
            }

            perimeters
                .entry(plant)
                .and_modify(|count| *count += fences)
                .or_insert(fences);
        }

        let mut output = Vec::new();

        for (plant, area) in areas {
            let &perimeter = perimeters.get(&plant).unwrap();

            output.push((area, perimeter));

            println!("Plant: {} Area {} Perimeter {}", plant, area, perimeter);
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
