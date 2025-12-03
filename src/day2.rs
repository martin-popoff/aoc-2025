const SEPARATOR: char = '-';

pub fn solve(input: Vec<String>, part2: bool) -> i64 {
    let mut result = 0;
    for combination in input.iter() {
        let (left, right) = combination.split_once(SEPARATOR).unwrap();

        // Wanted to use some clever algorithm, two hours later - brute force it is!
        for n in left.parse::<i64>().unwrap()..=right.parse::<i64>().unwrap() {
            let length = n.checked_ilog10().unwrap_or(0) + 1;

            if part2 {
                todo!();
                // for power in 1..=length / 2 {
                //     let mut prev: i64;
                //     for i in 0..=power {
                //         println!("{}", n / 10_i64.pow(power));
                //     }
                // }
            } else {
                if length % 2 != 0 {
                    continue;
                }
                let half = 10_i64.pow(length / 2);
                if n / half == n % half {
                    result += n;
                }
            }
        }
    }
    result
}
