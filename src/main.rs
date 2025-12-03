use crate::shared::*;

mod day1;
mod day2;
mod shared;

fn main() {
    day2();
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
