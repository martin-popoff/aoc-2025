use std::{
    fs::{self, File},
    io::{BufReader, prelude::*},
};

pub fn read_file(day: &str) -> String {
    fs::read_to_string(format!("./input/day{}.txt", day))
        .expect("Should have been able to read the file")
}

pub fn read_file_line_by_line(day: &str) -> Vec<String> {
    let file = File::open(format!("./input/day{}.txt", day)).expect("Could not open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect::<Vec<String>>()
}
