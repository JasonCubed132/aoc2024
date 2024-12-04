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
        println!("Success!");
        return true;
    }

    println!("Coord {:?} Reamining {:?}", current_coord, sequence);
    if !(grid[current_coord.1][current_coord.0].eq(&sequence[0])) {
        println!("Bad check on {:?}", sequence[0]);
        return false;
    }

    let (_, new_sequence) = sequence.split_at(1);

    let new_x = current_coord.0 as i32 + direction.0;
    let new_y = current_coord.1 as i32 + direction.1;

    if !((new_x >= 0 && new_x < grid[current_coord.1].len() as i32)
        && (new_y >= 0 && new_y < grid.len() as i32))
    {
        println!("Out of bounds {:?} {:?}", new_x, new_y);
        return false;
    }
    let new_coord = (new_x as usize, new_y as usize);

    spawn_checker(grid, new_sequence.to_vec(), new_coord, direction)
}

fn compute_day_a(input: &Vec<Vec<char>>) -> Result<i32> {
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
    let mut total = 0;
    for start_y_coord in 0..input.len() {
        for start_x_coord in 0..input[start_y_coord].len() {
            for direction in directions {
                if spawn_checker(
                    input,
                    "XMAX".chars().collect(),
                    (start_x_coord, start_y_coord),
                    direction,
                ) {
                    total += 1;
                }
            }
        }
    }

    Ok(total)
}

fn compute_day_b(input: &Vec<Vec<char>>) -> Result<i32> {
    todo!()
}
