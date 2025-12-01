const START: i32 = 50;

pub fn solve(input: Vec<String>, part2: bool) -> i32 {
    let mut count = START;
    let mut password = 0;
    for line in input.iter() {
        let direction = line.chars().next().unwrap();
        let number = &line[1..];
        let mut number_parsed = number.parse::<i32>().unwrap();
        // My other idea was going back to cpp age and subtracing char '0' over iterating
        // the remainder of the line

        if part2 {
            password += number_parsed / 100;
        }

        number_parsed %= 100;
        if direction == 'L' {
            count -= number_parsed;
        } else if direction == 'R' {
            count += number_parsed;
        }

        if count < 0 {
            count += 100;
            if part2 && count + number_parsed != 100 {
                password += 1;
            }
        } else if count > 100 {
            count -= 100;
            if part2 && count - number_parsed != 0 {
                password += 1;
            }
        } else if count % 100 == 0 {
            password += 1;
        }
    }

    password
}

pub fn solve_pretty(input: Vec<String>, part2: bool) -> i32 {
    let mut count = START;
    let mut password = 0;

    for line in &input {
        // No need for iter slightly more idomatic
        let (dir, number) = line.split_at(1); // Exactly what I was looking for in the docs
        let mut number_parsed: i32 = number.parse().unwrap(); // Type inference from parse

        if part2 {
            password += number_parsed / 100;
        }
        number_parsed %= 100;

        let old_count = count;
        match dir.as_bytes()[0] {
            // match is very strong and I keep forgetting it. Also cool conversion to char
            b'L' => count -= number_parsed,
            b'R' => count += number_parsed,
            _ => {}
        }

        // better than nested ifs for sure
        let landed_on_boundary = count % 100 == 0;
        let wrapped = count < 0 || count > 100;

        // Cool way to normalize to [0, 99]
        count = count.rem_euclid(100);

        // Updated password logic to reflect less nesting
        if landed_on_boundary || (wrapped && part2 && old_count != 0) {
            password += 1;
        }
    }

    password
}
