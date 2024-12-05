use anyhow::Result;

pub fn day_(input: String) -> Result<()> {
    let day_parsed_input = parse_day(input)?;
    let day_a_total = compute_day_a(&day_parsed_input)?;
    println!("Day 01 A Input result: {:?}", day_a_total);
    let day_b_total = compute_day_b(&day_parsed_input)?;
    println!("Day 01 B Input result: {:?}", day_b_total);

    Ok(())
}

fn parse_day(input: String) -> Result<_> {
    todo!();
}

fn compute_day_a(input: _) -> Result<_> {
    todo!();
}

fn compute_day_b(input: _) -> Result<_> {
    todo!();
}
