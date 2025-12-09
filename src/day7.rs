const START: char = 'S';
const EMPTY: char = '.';
const BEAM: char = '|';
const SPLIT: char = '^';

pub fn solve(input: Vec<String>, part2: bool) -> u64 {
    let mut copy = input.clone();
    let length = copy[0].len();
    // pascal triangle to sum the variants
    let mut triangle: Vec<Vec<u64>> = vec![vec![0; length]; input.len()];
    let mut result = 0;
    copy[1].replace_range(length / 2..length / 2 + 1, &BEAM.to_string());
    triangle[1][length / 2] = 1;
    // taken the "visual" approach this time
    for (i, line) in input.iter().enumerate() {
        for (j, char) in line.char_indices() {
            // hacky edge cases but thats life
            if char == SPLIT && copy[i - 1].chars().nth(j).unwrap() == BEAM {
                copy[i].replace_range(j - 1..j, &BEAM.to_string());
                copy[i].replace_range(j + 1..j + 2, &BEAM.to_string());
                if triangle[i][j + 1] == 0 {
                    triangle[i][j + 1] += triangle[i - 1][j + 1];
                }
                if triangle[i][j - 1] == 0 {
                    triangle[i][j - 1] += triangle[i - 1][j - 1];
                }
                triangle[i][j - 1] += triangle[i - 1][j];
                triangle[i][j + 1] += triangle[i - 1][j];
            }
            if i > 0 && triangle[i][j] == 0 && char == EMPTY {
                copy[i].replace_range(j..j + 1, &BEAM.to_string());
                triangle[i][j] = triangle[i - 1][j];
            }
        }
    }
    if part2 {
        result = triangle.last().unwrap().iter().sum();
    }
    result
}
