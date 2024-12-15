use std::fmt::Debug;

use anyhow::Result;

#[derive(Clone)]
enum DiskEntry {
    FreeSpace(u32),   // size
    File(usize, u32), // idx, size
}

impl Debug for DiskEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &DiskEntry::File(idx, size) => {
                for _ in 0..size {
                    write!(f, "{}", idx)?;
                }
            }
            &DiskEntry::FreeSpace(size) => {
                for _ in 0..size {
                    write!(f, ".")?;
                }
            }
        }

        Ok(())
    }
}

struct Disk {
    entries: Vec<DiskEntry>,
}

impl Disk {
    fn new(entries: Vec<DiskEntry>) -> Self {
        Self { entries }
    }

    fn from_fs_str(fs_str: String) -> Result<Self> {
        let disk_entries = fs_str
            .chars()
            .enumerate()
            .map(|(idx, val)| {
                let size = val.to_digit(10).unwrap() as u32;
                Ok(if idx % 2 == 0 {
                    DiskEntry::File(idx / 2, size)
                } else {
                    DiskEntry::FreeSpace(size)
                })
            })
            .collect::<Result<Vec<DiskEntry>>>()?;
        Ok(Self::new(disk_entries))
    }

    fn calculate_checksum(&self) -> u64 {
        let mut marker = 0;
        let mut total = 0;

        for entry in &self.entries {
            match entry {
                DiskEntry::FreeSpace(size) => {
                    marker += size;
                }
                DiskEntry::File(id, size) => {
                    let increment = (marker..marker + size).sum::<u32>() as u64 * (*id as u64);
                    total += increment;
                    marker += size;
                }
            }
        }
        total
    }

    fn defrag(&self) -> Self {
        let mut new_disk = Vec::new();
        let mut marker_forward = 0;
        let mut marker_reversed = 0;
        let mut reversed_disk = self.entries.clone();
        reversed_disk.reverse();

        while marker_forward < self.entries.len() - 1 - marker_reversed {
            // println!("Forward {} Reverse {} Len {}", marker_forward, marker_reversed, self.entries.len());
            match self.entries[marker_forward] {
                DiskEntry::File(idx, size) => {
                    // println!("Pushed file with id {} size {}", idx, size);
                    new_disk.push(DiskEntry::File(idx, size));
                    marker_forward += 1;
                }
                DiskEntry::FreeSpace(size) => {
                    // println!("Filling space of size {}", size);
                    let mut remaining_size = size;
                    while remaining_size > 0 {
                        let moving_file = &reversed_disk[marker_reversed];

                        match moving_file {
                            &DiskEntry::File(moving_idx, moving_size) => {
                                // println!("Inner - Evaluating file of id {} size {}", moving_idx, moving_size);
                                if moving_size <= remaining_size {
                                    new_disk.push(DiskEntry::File(moving_idx, moving_size));
                                    // println!("Inner - Pushed file with id {} size {}", moving_idx, moving_size);
                                    marker_reversed += 1;
                                    remaining_size -= moving_size;
                                } else {
                                    new_disk.push(DiskEntry::File(moving_idx, remaining_size));
                                    // println!("Inner - Pushed file with id {} size {}", moving_idx, remaining_size);
                                    reversed_disk[marker_reversed] =
                                        DiskEntry::File(moving_idx, moving_size - remaining_size);
                                    remaining_size = 0;
                                }
                            }
                            _ => {
                                marker_reversed += 1;
                            }
                        }
                    }

                    marker_forward += 1;
                }
            }
        }

        // We probably have a part file left at the end of the reverse evaluation
        match reversed_disk[marker_reversed] {
            DiskEntry::File(idx, size) => {
                if size > 0 {
                    new_disk.push(DiskEntry::File(idx, size))
                }
            }

            DiskEntry::FreeSpace(_) => {}
        }
        Disk::new(new_disk)
    }
}

impl Debug for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in &self.entries {
            write!(f, "{:?}", item)?;
        }

        Ok(())
    }
}

pub fn day09(input: String) -> Result<()> {
    let day_parsed_input = parse_day(input)?;
    let day_a_total = compute_day_a(&day_parsed_input)?;
    println!("Day 09 A Input result: {:?}", day_a_total);
    let day_b_total = compute_day_b(&day_parsed_input)?;
    println!("Day 09 B Input result: {:?}", day_b_total);

    Ok(())
}

fn parse_day(input: String) -> Result<Disk> {
    Disk::from_fs_str(input)
}

fn compute_day_a(input: &Disk) -> Result<u64> {
    // println!("{:?}", input);
    let defraged_disk = input.defrag();
    // println!("{:?}", defraged_disk);
    Ok(defraged_disk.calculate_checksum())
}

fn compute_day_b(input: &Disk) -> Result<usize> {
    todo!();
}
