mod day1;
mod shared;

fn main() {
    println!(
        "Day 1 solution: {}",
        day1::solve(shared::read_file_line_by_line("1"), false)
    );
    println!(
        "Day 1 part 2 solution: {}",
        day1::solve(shared::read_file_line_by_line("1"), true)
    );
}
