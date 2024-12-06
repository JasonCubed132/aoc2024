use core::num;
use std::cmp::Ordering;

use anyhow::Result;

pub fn day05(input: String) -> Result<()> {
    let day_parsed_input = parse_day(input)?;
    let day_a_total = compute_day_a(&day_parsed_input)?;
    println!("Day 05 A Input result: {:?}", day_a_total);
    let day_b_total = compute_day_b(&day_parsed_input)?;
    println!("Day 05 B Input result: {:?}", day_b_total);

    Ok(())
}

#[derive(PartialEq, Copy, Clone)]
struct Rule {
    left: u32,
    right: u32,
}

struct Num {
    constraints: Vec<Rule>,
    inner: u32,
}

impl PartialOrd for Num {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        for rule in &self.constraints {
            if self.inner == rule.left && other.inner == rule.right {
                return Some(Ordering::Less);
            }
            if self.inner == rule.right && other.inner == rule.left {
                return Some(Ordering::Greater);
            }
        }
        None
    }
}

impl PartialEq for Num {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Ord for Num {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Num {}

fn parse_day(input: String) -> Result<(Vec<(u32, u32)>, Vec<Vec<u32>>)> {
    let (rules_str, updates_str) = input.split_once("\r\n\r\n").unwrap();

    let rules: Vec<(u32, u32)> = rules_str
        .lines()
        .map(|x| {
            let res: Vec<u32> = x
                .split("|")
                .map(|y| {
                    let res = y.parse::<u32>()?;
                    Ok::<u32, anyhow::Error>(res)
                })
                .collect::<Result<Vec<u32>>>()?;
            Ok::<(u32, u32), anyhow::Error>((res[0], res[1]))
        })
        .collect::<Result<Vec<(u32, u32)>>>()?;

    let updates = updates_str
        .lines()
        .map(|x| {
            let res = x
                .split(",")
                .map(|y| {
                    let res = y.parse::<u32>()?;
                    Ok::<u32, anyhow::Error>(res)
                })
                .collect::<Result<Vec<u32>>>()?;
            Ok(res)
        })
        .collect::<Result<Vec<Vec<u32>>>>()?;

    Ok((rules, updates))
}

fn compute_day_a(input: &(Vec<(u32, u32)>, Vec<Vec<u32>>)) -> Result<u32> {
    let (rules, updates) = input;
    let rules_formatted: Vec<Rule> = rules
        .into_iter()
        .map(|(left, right)| Rule {
            left: *left,
            right: *right,
        })
        .collect();
    Ok(updates
        .into_iter()
        .map(|vec| {
            vec.into_iter()
                .map(|item| Num {
                    constraints: rules_formatted.clone(),
                    inner: *item,
                })
                .collect::<Vec<Num>>()
        })
        .filter(|vec| vec.is_sorted())
        .map(|vec| vec[(vec.len() - 1) / 2].inner)
        .sum())
}

fn compute_day_b(input: &(Vec<(u32, u32)>, Vec<Vec<u32>>)) -> Result<u32> {
    let (rules, updates) = input;
    let rules_formatted: Vec<Rule> = rules
        .into_iter()
        .map(|(left, right)| Rule {
            left: *left,
            right: *right,
        })
        .collect();
    Ok(updates
        .into_iter()
        .map(|vec| {
            vec.into_iter()
                .map(|item| Num {
                    constraints: rules_formatted.clone(),
                    inner: *item,
                })
                .collect::<Vec<Num>>()
        })
        .filter(|vec| !vec.is_sorted())
        .map(|mut vec| {
            vec.sort();
            vec
        })
        .map(|vec| vec[(vec.len() - 1) / 2].inner)
        .sum())
}
