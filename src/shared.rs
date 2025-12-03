use std::{
    fs::File,
    io::{BufReader, prelude::*},
};

pub fn read_file_line_by_line(day: &str) -> Vec<String> {
    let file = File::open(format!("./input/day{}.txt", day)).expect("Could not open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect::<Vec<String>>()
}

pub fn read_file_first_line_with_delimiter(day: &str, delimiter: char) -> Vec<String> {
    let file = File::open(format!("./input/day{}.txt", day)).expect("Could not open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .next()
        .expect("Could not read line")
        .unwrap()
        .split(delimiter)
        .map(String::from)
        .collect()
}
