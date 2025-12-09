const SEPARATOR: char = '-';

pub fn solve(input: Vec<String>, part2: bool) -> u64 {
    let mut result = 0;
    for combination in input.iter() {
        let (left, right) = combination.split_once(SEPARATOR).unwrap();

        // Wanted to use some clever algorithm, two hours later - brute force it is!
        for n in left.parse::<u64>().unwrap()..=right.parse::<u64>().unwrap() {
            let length = n.checked_ilog10().unwrap_or(0) + 1;

            if part2 {
                let mut same = false;
                for i in 1..=length / 2 {
                    if same {
                        break;
                    }
                    if length % i != 0 {
                        continue;
                    }
                    let mut last = n % 10_u64.pow(length) / 10_u64.pow(length - i);
                    let total = length / i;
                    let mut keep = true;
                    for j in 1..total {
                        let num = n % 10_u64.pow(length - j * i) / 10_u64.pow(length - j * i - i);
                        if num != last {
                            keep = false;
                            break;
                        }
                        last = num;
                    }
                    if keep {
                        same = true;
                    }
                }
                if same {
                    result += n;
                }
            } else {
                if length % 2 != 0 {
                    continue;
                }
                let half = 10_u64.pow(length / 2);
                if n / half == n % half {
                    result += n;
                }
            }
        }
    }
    result
}
