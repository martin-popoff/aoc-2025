const EMPTY: char = '.';

pub fn solve(input: Vec<String>, part2: bool) -> u64 {
    let mut res = 0;
    if part2 {
        let (mut copy, mut result) = remove(input);
        res += result;
        while result > 0 {
            (copy, result) = remove(copy);
            res += result;
        }
    } else {
        let (_, result) = remove(input);
        res = result;
    }
    res
}

// OK so I have so many ideas of ways to improve but too late and lazy
fn remove(input: Vec<String>) -> (Vec<String>, u64) {
    let mut copy = input.clone();
    let mut result = 0;
    for (i, line) in input.iter().enumerate() {
        for (j, _) in line.char_indices().filter(|&(_, char)| char != EMPTY) {
            let mut total: u8 = 0;
            if i > 0 && j > 0 && input[i - 1].chars().nth(j - 1).unwrap() != EMPTY {
                total += 1;
            }
            if i > 0 && input[i - 1].chars().nth(j).unwrap() != EMPTY {
                total += 1;
            }
            if i > 0 && j < line.len() - 1 && input[i - 1].chars().nth(j + 1).unwrap() != EMPTY {
                total += 1;
            }
            if j > 0 && input[i].chars().nth(j - 1).unwrap() != EMPTY {
                total += 1;
            }
            if j < line.len() - 1 && input[i].chars().nth(j + 1).unwrap() != EMPTY {
                total += 1;
            }
            if i < input.len() - 1 && j > 0 && input[i + 1].chars().nth(j - 1).unwrap() != EMPTY {
                total += 1;
            }
            if i < input.len() - 1 && input[i + 1].chars().nth(j).unwrap() != EMPTY {
                total += 1;
            }
            if i < input.len() - 1
                && j < line.len() - 1
                && input[i + 1].chars().nth(j + 1).unwrap() != EMPTY
            {
                total += 1;
            }
            if total < 4 {
                result += 1;
                copy[i].replace_range(j..j + 1, ".");
            }
        }
    }
    (copy, result)
}
