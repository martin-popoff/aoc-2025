const MULTI: char = '*';
const ADD: char = '+';

pub fn solve(input: Vec<Vec<String>>, part2: bool) -> u64 {
    let mut result = 0;
    for (i, row) in input[0].iter().enumerate() {
        let action = input[input.len() - 1][i].chars().next().unwrap();
        let mut sum = if action == MULTI { 1 } else { 0 };
        for j in 0..input.len() - 1 {
            if action == MULTI {
                sum *= input[j][i].parse::<u64>().unwrap();
            } else if action == ADD {
                sum += input[j][i].parse::<u64>().unwrap();
            }
        }
        println!("{}", sum);
        result += sum;
    }

    result
}
