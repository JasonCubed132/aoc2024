use anyhow::Result;

pub fn day05(input: String) -> Result<()> {
    let day_parsed_input = parse_day(input)?;
    let day_a_total = compute_day_a(&day_parsed_input)?;
    println!("Day 05 A Input result: {:?}", day_a_total);
    let day_b_total = compute_day_b(&day_parsed_input)?;
    println!("Day 05 B Input result: {:?}", day_b_total);

    Ok(())
}

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

    Ok(updates
        .into_iter()
        .map(|update| {
            let res = rules
                .into_iter()
                .map(|rule| {
                    update.into_iter().fold(None, |acc, elem| match acc {
                        Some(false) => Some(false),
                        Some(true) => {
                            if *elem == rule.0 {
                                Some(false)
                            } else {
                                Some(true)
                            }
                        }
                        None => {
                            if *elem == rule.1 {
                                Some(true)
                            } else {
                                None
                            }
                        }
                    })
                })
                .fold(true, |acc, result| match acc {
                    false => false,
                    true => match result {
                        Some(true) => true,
                        Some(false) => false,
                        None => true,
                    },
                });

            (res, update[(update.len() - 1) / 2])
        })
        .filter(|(res, _)| *res)
        .map(|(_, val)| val)
        .sum())
}

fn compute_day_b(input: &(Vec<(u32, u32)>, Vec<Vec<u32>>)) -> Result<u32> {
    todo!();
}
