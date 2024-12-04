use std::iter::zip;

use anyhow::Result;

use super::list_ops::count_items_in_list;

pub fn day01(input: String) -> Result<()> {
    let day_parsed_input = parse_day(input)?;
    let day_a_total = compute_day_a(&day_parsed_input)?;
    println!("Day 01 A Input result: {:?}", day_a_total);
    let day_b_total = compute_day_b(&day_parsed_input)?;
    println!("Day 01 B Input result: {:?}", day_b_total);

    Ok(())
}

fn parse_day(input: String) -> Result<(Vec<i32>, Vec<i32>)> {
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

fn compute_day_a(input: &(Vec<i32>, Vec<i32>)) -> Result<i32> {
    let (mut list_1, mut list_2) = input.clone();

    list_1.sort();
    list_2.sort();

    let total = zip(list_1, list_2).map(|(a, b)| (a - b).abs()).sum();
    Ok(total)
}

fn compute_day_b(input: &(Vec<i32>, Vec<i32>)) -> Result<i32> {
    let (list_1, list_2) = input;

    let list_2_counts = count_items_in_list(list_2);

    let total = list_1
        .iter()
        .map(|item| item * list_2_counts.get(&item).unwrap_or(&0))
        .sum();

    Ok(total)
}
