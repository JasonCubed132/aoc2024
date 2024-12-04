use anyhow::Result;

pub fn day04(input: String) -> Result<()> {
    let day_parsed_input = parse_day(input)?;
    let day_a_total = compute_day_a(&day_parsed_input)?;
    println!("Day 4 A Input result: {:?}", day_a_total);
    let day_b_total = compute_day_b(&day_parsed_input)?;
    println!("Day 4 B Input result: {:?}", day_b_total);

    Ok(())
}

fn parse_day(input: String) -> Result<Vec<Vec<char>>> {
    Ok(input
        .lines()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect())
}

fn spawn_checker(
    grid: &Vec<Vec<char>>,
    sequence: Vec<char>,
    current_coord: (usize, usize),
    direction: (i32, i32),
) -> bool {
    if sequence.len() == 0 {
        return true;
    }

    if !(grid[current_coord.1][current_coord.0].eq(&sequence[0])) {
        return false;
    }

    let (_, new_sequence) = sequence.split_at(1);

    let new_x = current_coord.0 as i32 + direction.0;
    let new_y = current_coord.1 as i32 + direction.1;

    if new_sequence.len() > 0
        && !((new_x >= 0 && new_x < grid[current_coord.1].len() as i32)
            && (new_y >= 0 && new_y < grid.len() as i32))
    {
        return false;
    }
    let new_coord = (new_x as usize, new_y as usize);

    spawn_checker(grid, new_sequence.to_vec(), new_coord, direction)
}

fn check_for_string(
    grid: &Vec<Vec<char>>,
    string: String,
) -> Result<Vec<((usize, usize), (i32, i32))>> {
    /*
       8 cases to evalutate:
       - Horizontal forward
       - Horizontal backward
       - Vertical forward
       - Vertical backward
       - Leading forward
       - Leading backward
       - Trailing forward
       - Trailing backward
    */

    let directions = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    let mut output = Vec::new();
    for start_y_coord in 0..grid.len() {
        for start_x_coord in 0..grid[start_y_coord].len() {
            let start_coord = (start_x_coord, start_y_coord);
            for direction in directions {
                if spawn_checker(grid, string.chars().collect(), start_coord, direction) {
                    output.push((start_coord, direction));
                }
            }
        }
    }

    Ok(output)
}

fn compute_day_a(input: &Vec<Vec<char>>) -> Result<usize> {
    Ok(check_for_string(input, "XMAS".to_string())?.len())
}

fn compute_day_b(input: &Vec<Vec<char>>) -> Result<i32> {
    let results = check_for_string(input, "MAS".to_string())?;

    println!("{}", results.len());
    // Move all pointers to the A in MAS
    let mut a_points = Vec::new();

    let mut count = 0;
    for (start, dir) in results {
        let new_x = start.0 as i32 + dir.0;
        let new_y = start.1 as i32 + dir.1;

        let new_coord = (new_x, new_y);
        if a_points.contains(&new_coord) {
            count += 1;
        } else {
            a_points.push(new_coord);
        }
    }
    Ok(count)
}
