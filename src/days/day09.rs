use anyhow::Result;

enum DiskEntry {
    FreeSpace(usize),
    File(usize, usize),
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
                let size = val.to_digit(10).unwrap() as usize;
                Ok(if idx % 2 == 0 {
                    DiskEntry::File(idx / 2, size)
                } else {
                    DiskEntry::FreeSpace(size)
                })
            })
            .collect::<Result<Vec<DiskEntry>>>()?;
        Ok(Self::new(disk_entries))
    }

    fn calculate_checksum(&self) -> usize {
        let mut marker = 0;
        let mut total = 0;

        for entry in &self.entries {
            match entry {
                DiskEntry::FreeSpace(size) => {
                    marker += size;
                }
                DiskEntry::File(id, size) => {
                    let increment = (marker..marker + size).sum::<usize>() * id;
                    total += increment;
                    marker += size;
                }
            }
        }
        total
    }

    fn defrag(&self) -> Self {
        todo!()
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

fn compute_day_a(input: &Disk) -> Result<usize> {
    let defraged_disk = input.defrag();
    Ok(defraged_disk.calculate_checksum())
}

fn compute_day_b(input: &Disk) -> Result<usize> {
    todo!();
}
