use std::{
    fs::read_to_string,
    io::{self},
};
const INPUT_PATH: &str = "resources/";

pub fn get_input(day: u8) -> io::Result<String> {
    let path = [INPUT_PATH, &day.to_string(), ".txt"].concat();
    let input = read_to_string(path)?;
    Ok(input)
}
