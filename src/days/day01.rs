use std::iter::zip;

use anyhow::Result;

use super::list_ops::count_items_in_list;

pub fn parse_day01(input: String) -> Result<(Vec<i32>, Vec<i32>)> {
    let as_lines = input.lines();

    let mut list_1 = Vec::new();
    let mut list_2 = Vec::new();
    for line in as_lines {
        let parts = line.split("   ");
        let parts_list: Vec<&str> = parts.collect();
        let part_1: i32 = parts_list[0].parse()?;
        let part_2: i32 = parts_list[1].parse()?;
        list_1.push(part_1);
        list_2.push(part_2);
    }

    Ok((list_1, list_2))
}

pub fn compute_day01a(input: &(Vec<i32>, Vec<i32>)) -> Result<i32> {
    let (mut list_1, mut list_2) = input.clone();

    list_1.sort();
    list_2.sort();

    let lists = zip(list_1, list_2);
    let diff = lists.map(|(a, b)| { (a - b).abs() });
    let total: i32 = diff.sum();

    Ok(total)
}

pub fn compute_day01b(input: &(Vec<i32>, Vec<i32>)) -> Result<i32> {
    let (list_1, list_2) = input;

    let list_2_counts = count_items_in_list(list_2);

    let mut total = 0;

    for item in list_1 {
        match list_2_counts.get(&item) {
            Some(x) => {
                total += item * x;
            }
            None => {}
        }
    }

    Ok(total)
}