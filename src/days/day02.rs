use anyhow::Result;

pub fn day02(input: String) -> Result<()> {
    let day02_parsed_input = parse_day02(input)?;
    let day02a_total = compute_day02a(&day02_parsed_input)?;
    println!("Day 02 A Input result: {:?}", day02a_total);
    let day02b_total = compute_day02b(&day02_parsed_input)?;
    println!("Day 02 B Input result: {:?}", day02b_total);

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

fn compute_report(input: &Vec<i32>) -> (i32, i32, i32, &Vec<i32>) {
    input
        .windows(2)
        .fold((0, 0, 0, input), |(cnt, asc, dec, report), window| {
            let diff_valid =
                (window[0] - window[1]).abs() <= 3 && (window[0] - window[1]).abs() >= 1;
            if diff_valid {
                if window[0] < window[1] {
                    (cnt + 1, asc + 1, dec, report)
                } else {
                    (cnt + 1, asc, dec + 1, report)
                }
            } else {
                (cnt + 1, asc, dec, report)
            }
        })
}

fn compute_reports(input: &Vec<Vec<i32>>) -> Vec<(i32, i32, i32, &Vec<i32>)> {
    input.into_iter().map(compute_report).collect()
}

pub fn compute_day02a(input: &Vec<Vec<i32>>) -> Result<i32> {
    Ok(compute_reports(input)
        .into_iter()
        .fold(0, |count, (cnt, asc, dec, _)| {
            if cnt == asc || cnt == dec {
                count + 1
            } else {
                count
            }
        }))
}

fn test_remove(report: &Vec<i32>, index: usize) -> bool {
    let mut test_report = report.clone();
    test_report.remove(index);
    let (cnt, asc, dec, _) = compute_report(&test_report);

    dec == cnt || asc == cnt
}

fn test_removals(report: &Vec<i32>, func: &dyn Fn(&(usize, &[i32])) -> bool) -> usize {
    report
        .windows(2)
        .enumerate()
        .filter(func)
        .map(|window| window.0)
        .filter(|index| test_remove(report, index + 0) || test_remove(report, index + 1))
        .collect::<Vec<_>>()
        .len()
}

fn test_ascending(window: &(usize, &[i32])) -> bool {
    !(1..3).contains(&(window.1[1] - window.1[0]))
}
fn test_descending(window: &(usize, &[i32])) -> bool {
    !(1..3).contains(&(window.1[0] - window.1[1]))
}

pub fn compute_day02b(input: &Vec<Vec<i32>>) -> Result<i32> {
    let result: i32 = compute_reports(input)
        .into_iter()
        .map(|(cnt, asc, dec, report)| {
            if cnt == asc || cnt == dec {
                1
            } else if cnt - 1 == asc {
                let count = test_removals(report, &test_ascending);
                if count > 0 {
                    1
                } else {
                    0
                }
            } else if cnt - 1 == dec {
                let count = test_removals(report, &test_descending);
                if count > 0 {
                    1
                } else {
                    0
                }
            } else {
                print!("Asc {} Dec {} ", asc, dec);
                println!("Unlikely {:?}", report);
                0
            }
        })
        .sum();

    Ok(result)
}
