use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::{anyhow, Result};

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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn to_delta(&self) -> Delta {
        match self {
            Self::North => Delta::new(0, -1),
            Self::East => Delta::new(1, 0),
            Self::South => Delta::new(0, 1),
            Self::West => Delta::new(-1, 0)
        }
    }

    fn from_delta(delta: Delta) -> Result<Self> {
        if delta.get_x() == 0 && delta.get_y() != 0 {
            if delta.get_y() > 0 {
                Ok(Self::North)
            } else {
                Ok(Self::South)
            }
        } else if delta.get_x() != 0 && delta.get_y() == 0 {
            if delta.get_x() > 0 {
                Ok(Self::East)
            } else {
                Ok(Self::West)
            }
        } else {
            Err(anyhow!("Can't figure out a sensible direction from delta {:?}!", delta))
        }
    }

    fn turn_90(&self) -> Self {
        match self {
            Self::North =>  Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Axis {
    X,
    Y
}

impl Axis {
    fn from_direction(direction: &Direction) ->Self {
        match direction {
            Direction::North => Self::Y,
            Direction::East => Self::X,
            Direction::South => Self::Y,
            Direction::West => Self::X
        }
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
        // Explored if we have searched its neighbours
        let mut globally_explored_coords = HashSet::new();

        let mut new_region_coords_to_search = Vec::new();
        new_region_coords_to_search.push(start);

        while new_region_coords_to_search.len() > 0 {
            // Start exploration of new region based on cell we
            // know is within that region.
            let new_region_start_coord = new_region_coords_to_search.pop().unwrap();

            if globally_explored_coords.contains(&new_region_start_coord) {
                continue;
                // Skip as coord has been queued as a new starting point despite being discovered
                // as an existing region.
            }

            // println!("Starting new region at {}", new_region_start_coord);

            let mut coords_in_region_to_search = Vec::new();
            coords_in_region_to_search.push(new_region_start_coord);

            let mut fences: u32 = 0;
            let mut area: u32 = 0;

            // Discover new cells within region by checking neighbours.
            while coords_in_region_to_search.len() > 0 {
                let explore_anchor_coord = coords_in_region_to_search.pop().unwrap();
                let explore_anchor_cell = self.garden.get_cell_contents(&explore_anchor_coord)?;

                globally_explored_coords.insert(explore_anchor_coord);
                area += 1;

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
                        if !globally_explored_coords.contains(&coord_being_explored)
                            && !coords_in_region_to_search.contains(&coord_being_explored)
                        {
                            // println!("Pushed to local {}", coord_being_explored);
                            coords_in_region_to_search.push(coord_being_explored);
                        }
                    } else {
                        fences += 1;
                        if !globally_explored_coords.contains(&coord_being_explored)
                            && !new_region_coords_to_search.contains(&coord_being_explored)
                        {
                            // println!("Pushed to global {} {}", coord_being_explored, self.garden.get_cell_contents(&coord_being_explored)?);
                            new_region_coords_to_search.push(coord_being_explored);
                        }
                    }
                }
            }

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
    // let _ = input
    //     .calculate_areas_and_perimetres()?
    //     .iter()
    //     .map(|(_, perimeter)| -> Result<()> {
    //         // Sort fences by axis to reduce the processing later stages do.
    //         let mut direction_bins = HashMap::new();

    //         for fence in perimeter {
    //             direction_bins.entry(fence.get_axis()?).and_modify(
    //                 |vec: &mut Vec<&Fence>| {
    //                     vec.push(fence);
    //                 }
    //             ).or_insert({
    //                 let mut vec = Vec::new(); 
    //                 vec.push(fence);
    //                 vec
    //             });
    //         }

    //         for fences in direction_bins.values_mut() {
    //             if fences.len() == 0 {
    //                 continue;
    //             }
    //             let mut continous_fences = Vec::new();
    //             let mut current_fence = fences.pop().unwrap();
    //             continous_fences.push(current_fence);
    //         }

    //         Ok(())
    //     });
    // todo!();
    // Ok(total)
}
