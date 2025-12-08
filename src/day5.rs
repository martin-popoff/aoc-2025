pub fn solve(input: (Vec<(u64, u64)>, Vec<u64>), part2: bool) -> u64 {
    if part2 {
        return part2_solve(input.0);
    }

    let mut fresh_count = 0;
    for id in input.1 {
        for (lower, upper) in &input.0 {
            if &id >= lower && &id <= upper {
                fresh_count += 1;
                break;
            }
        }
    }
    fresh_count
}

fn part2_solve(mut input: Vec<(u64, u64)>) -> u64 {
    let mut result = 0;
    input.sort();

    let mut cursor = 0;
    for &(lower, upper) in input.iter() {
        if cursor < upper {
            let min = lower.max(cursor + 1);
            result += upper - min + 1;
            cursor = upper;
        }
    }
    result
}
