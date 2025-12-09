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

pub fn read_file_from_two_parts(day: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let file = File::open(format!("./input/day{}.txt", day)).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut lines = reader
        .lines()
        .map(|line| line.expect("Could not read line"));

    let group1: Vec<(u64, u64)> = lines
        .by_ref()
        .map_while(|line| {
            if line.is_empty() {
                None
            } else {
                let (lower, upper) = line.split_once('-').unwrap();
                Some((lower.parse().unwrap(), upper.parse().unwrap()))
            }
        })
        .collect();

    let group2: Vec<u64> = lines.map(|line| line.parse::<u64>().unwrap()).collect();
    (group1, group2)
}

pub fn read_file_line_by_line_coordinates(day: &str) -> Vec<(i64, i64, i64)> {
    let file = File::open(format!("./input/day{}.txt", day)).expect("Could not open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line
                .expect("Could not read line")
                .split(',')
                .map(|c| c.parse::<i64>().unwrap())
                .collect();
            (coords[0], coords[1], coords[2])
        })
        .collect()
}
