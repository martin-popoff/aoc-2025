const MULTI: char = '*';
const ADD: char = '+';

pub fn solve(input: Vec<String>, part2: bool) -> u64 {
    let mut result = 0;
    let lengths = input[input.len() - 1]
        .split(|char| char != ' ')
        .collect::<Vec<&str>>();
    let mut cursor: usize = 0;
    for &column in lengths.iter().skip(1) {
        let action = input[input.len() - 1].chars().nth(cursor).unwrap();
        let mut numbers: Vec<&str> = vec![];
        let other_numbers: Vec<String>;
        // Probably inefficient but experimenting with iterators and their methods
        for row in input.iter().rev().skip(1).rev() {
            let string = &row[cursor..cursor + column.len()];
            numbers.push(string);
        }
        cursor += column.len() + 1;

        if part2 {
            other_numbers = (0..column.len())
                .rev()
                .map(|i| numbers.iter().map(|n| n.chars().nth(i).unwrap()).collect())
                .collect();

            numbers = other_numbers.iter().map(|s| s.as_str()).collect();
        }
        result += if action == ADD {
            numbers
                .iter()
                .map(|n| n.trim().parse::<u64>().unwrap())
                .sum::<u64>()
        } else {
            numbers
                .iter()
                .map(|n| n.trim().parse::<u64>().unwrap())
                .product::<u64>()
        };
    }

    result
}

// I know Opus is going to have a field day with this but anyways:
#[allow(dead_code)]
fn solve_pretty(input: Vec<String>, part2: bool) -> u64 {
    let footer = input.last().unwrap();
    let data_rows = &input[..input.len() - 1];

    let mut result = 0;
    let mut cursor = 0;

    for column in footer.split(|c| c != ' ').skip(1) {
        let action = footer.chars().nth(cursor).unwrap();
        let width = column.len();

        let mut numbers: Vec<&str> = data_rows
            .iter()
            .map(|row| &row[cursor..cursor + width])
            .collect();

        let transposed: Vec<String>;
        if part2 {
            transposed = (0..width)
                .rev()
                .map(|i| numbers.iter().map(|n| n.chars().nth(i).unwrap()).collect())
                .collect();
            numbers = transposed.iter().map(|s| s.as_str()).collect();
        }

        let parsed = numbers.iter().map(|n| n.trim().parse::<u64>().unwrap());
        result += match action {
            ADD => parsed.sum::<u64>(),
            MULTI => parsed.product(),
            _ => unreachable!(),
        };

        cursor += width + 1;
    }

    result
}
