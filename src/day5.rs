use std::cmp;

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

    let mut cursor = &0;
    for (i, (lower, upper)) in input.iter().enumerate() {
        if cursor > upper {
            continue;
        }
        // let min = cmp::max(cursor, lower);
        let min = if cursor >= lower {
            &(cursor - 1)
        } else {
            lower
        };
        let max = cmp::max(cursor, upper);

        result += max - min;
        cursor = max;

        // The next range is contained
        // if max > &input[i + 1].1 {
        // result += max - min;
        // cursor = max;
        // The ranges do not intersect
        // } else if max < &input[i + 1].0 {
        // result += max - min;
        // cursor = max;
        // Intersect
        // } else {
        // result += max - min;
        // cursor = max;
        // }
    }
    result
}
