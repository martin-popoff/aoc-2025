const BASE: u32 = 10;

pub fn solve(input: Vec<String>, battery_size: usize) -> u64 {
    let mut result: u64 = 0;
    for line in input.iter() {
        let mut max: Vec<u64> = vec![0; battery_size];
        let nums = line
            .chars()
            .map(|char| char.to_digit(BASE).unwrap())
            .collect::<Vec<u32>>();
        let mut last_position = 0;
        // Clippy suggests some inline code optimizations might check it out later
        for i in 0..battery_size {
            for j in last_position..nums.len() - battery_size + i + 1 {
                if nums[j] > max[i] as u32 {
                    last_position = j + 1;
                    max[i] = nums[j] as u64;
                }
            }
        }
        result += max.iter().fold(0, |acc, elem| acc * 10 + elem);
    }
    result
}

#[allow(dead_code)]
pub fn solve_pretty(input: Vec<String>, battery_size: usize) -> u64 {
    let mut result: u64 = 0;
    for line in input.iter() {
        let mut max: Vec<u64> = vec![0; battery_size];
        let nums: Vec<u32> = line.chars().map(|c| c.to_digit(BASE).unwrap()).collect();

        let mut last_position = 0;

        // I mean yeah I guess. Although it seems harder to understand if not familiar with rust syntax sugar
        for (i, max_elem) in max.iter_mut().enumerate() {
            for (j, &num) in nums
                .iter()
                .enumerate()
                .take(nums.len() - battery_size + i + 1)
                .skip(last_position)
            {
                if num > *max_elem as u32 {
                    last_position = j + 1;
                    *max_elem = num as u64;
                }
            }
        }

        result += max.iter().fold(0, |acc, elem| acc * 10 + elem);
    }
    result
}
