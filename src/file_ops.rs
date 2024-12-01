use anyhow::Result;
use std::io::Write;
use std::{fs, path::Path};

pub fn read_file(path: &Path) -> Result<String> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

pub fn get_example_input_path(day: i32) -> Result<String> {
    let mut output = Vec::new();
    write!(&mut output, "example_inputs/day{:0>2}.txt", day.to_string())?;
    let string = String::from_utf8(output)?;
    Ok(string)
}

pub fn get_input_path(day: i32) -> Result<String> {
    let mut output = Vec::new();
    write!(&mut output, "inputs/day{:0>2}.txt", day.to_string())?;
    let string = String::from_utf8(output)?;
    Ok(string)
}

pub fn read_example_input(day: i32) -> Result<String> {
    let path = get_example_input_path(day)?;
    read_file(Path::new(&path))
}

pub fn read_input(day: i32) -> Result<String> {
    let path = get_input_path(day)?;
    read_file(Path::new(&path))
}
