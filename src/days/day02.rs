use std::iter::zip;

use anyhow::Result;

pub fn day02(input: String) -> Result<()> {
    let day02_parsed_input = parse_day02(input)?;
    let day02a_total = compute_day02a(&day02_parsed_input)?;
    println!("Day 02 A Input result: {:?}", day02a_total);
    // let day02b_total = compute_day01b(&day02_parsed_input)?;
    // println!("Day 02 B Input result: {:?}", day02b_total);

    Ok(())
}

pub fn parse_day02(input: String) -> Result<Vec<Vec<i32>>> {
    input
        .lines()
        .map(|x| {
            x.split(" ")
                .map(|y| {
                    let res = y.parse::<i32>()?;
                    Ok(res)
                })
                .collect::<Result<Vec<i32>>>()
        })
        .collect::<Result<Vec<Vec<i32>>>>()
}

pub fn compute_day02a(input: &Vec<Vec<i32>>) -> Result<i32> {
    let mut count = 0;

    for report in input {
        let (asc, dec) = report
            .windows(2)
            .map(|window| {
                let diff_valid =
                    (window[0] - window[1]).abs() <= 3 && (window[0] - window[1]).abs() >= 1;
                if diff_valid {
                    if window[0] < window[1] {
                        Some(true)
                    } else {
                        Some(false)
                    }
                } else {
                    None
                }
            })
            .fold((true, true), |(asc, dec), item| {
                if item.is_none() {
                    (false, false)
                } else {
                    if item.unwrap() {
                        (asc, false)
                    } else {
                        (false, dec)
                    }
                }
            });

        if asc || dec {
            count += 1;
        }
    }

    Ok(count)
}

// pub fn compute_day02b(input: &(Vec<i32>, Vec<i32>)) -> Result<i32> {
//     let (list_1, list_2) = input;

//     let list_2_counts = count_items_in_list(list_2);

//     let total = list_1
//         .iter()
//         .map(|item| item * list_2_counts.get(&item).unwrap_or(&0))
//         .sum();

//     Ok(total)
// }
