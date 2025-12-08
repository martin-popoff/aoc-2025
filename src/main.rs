use crate::shared::*;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod shared;

fn main() {
    day6();
}

#[allow(dead_code)]
fn day1() {
    println!(
        "Day 1 solution: {}",
        day1::solve(read_file_line_by_line("1"), false)
    );
    println!(
        "Day 1 part 2 solution: {}",
        day1::solve(read_file_line_by_line("1"), true)
    );
}

#[allow(dead_code)]
fn day2() {
    println!(
        "Day 2 solution: {}",
        day2::solve(read_file_first_line_with_delimiter("2", ','), false)
    );
    println!(
        "Day 2 part 2 solution: {}",
        day2::solve(read_file_first_line_with_delimiter("2", ','), true)
    );
}

#[allow(dead_code)]
fn day3() {
    println!(
        "Day 3 solution: {}",
        day3::solve(read_file_line_by_line("3"), 2)
    );
    println!(
        "Day 3 part 2 solution: {}",
        day3::solve(read_file_line_by_line("3"), 12)
    );
}

#[allow(dead_code)]
fn day4() {
    println!(
        "Day 4 solution: {}",
        day4::solve(read_file_line_by_line("4"), false)
    );
    println!(
        "Day 4 part 2 solution: {}",
        day4::solve(read_file_line_by_line("4"), true)
    );
}

#[allow(dead_code)]
fn day5() {
    println!(
        "Day 5 solution: {}",
        day5::solve(read_file_from_two_parts("5"), false)
    );
    println!(
        "Day 5 part 2 solution: {}",
        day5::solve(read_file_from_two_parts("5"), true)
    );
}

fn day6() {
    // println!(
    //     "Day 6 solution: {}",
    //     day6::solve(read_file_line_by_line_trimmed_whitespace("6"), false)
    // );
    println!(
        "Day 6 part 2 solution: {}",
        day6::solve(read_file_line_by_line_trimmed_whitespace("6_example"), true)
    );
}
