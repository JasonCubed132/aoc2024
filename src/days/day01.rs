use std::iter::zip;

use anyhow::Result;

use super::list_ops::count_items_in_list;

pub fn day01(input: String) -> Result<()> {
    let day01_parsed_input = parse_day01(input)?;
    let day01a_total = compute_day01a(&day01_parsed_input)?;
    println!("Day 01 A Input result: {:?}", day01a_total);
    let day01b_total = compute_day01b(&day01_parsed_input)?;
    println!("Day 01 B Input result: {:?}", day01b_total);

    Ok(())
}

pub fn parse_day01(input: String) -> Result<(Vec<i32>, Vec<i32>)> {
    (input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(a, b)| -> Result<(i32, i32)> {
            let parsed_a = a.parse::<i32>()?;
            let parsed_b = b.parse::<i32>()?;
            Ok((parsed_a, parsed_b))
        })
        .collect::<Result<Vec<_>>>())
    .map(|x| x.into_iter().unzip())
}

pub fn compute_day01a(input: &(Vec<i32>, Vec<i32>)) -> Result<i32> {
    let (mut list_1, mut list_2) = input.clone();

    list_1.sort();
    list_2.sort();

    let total = zip(list_1, list_2).map(|(a, b)| (a - b).abs()).sum();
    Ok(total)
}

pub fn compute_day01b(input: &(Vec<i32>, Vec<i32>)) -> Result<i32> {
    let (list_1, list_2) = input;

    let list_2_counts = count_items_in_list(list_2);

    let total = list_1
        .iter()
        .map(|item| item * list_2_counts.get(&item).unwrap_or(&0))
        .sum();

    Ok(total)
}
