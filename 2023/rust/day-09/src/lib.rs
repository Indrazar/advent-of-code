fn all_zero(input: &Vec<i64>) -> bool {
    for num in input {
        if *num != 0 {
            return false;
        }
    }
    true
}

fn process_line(line: &str) -> Vec<Vec<i64>> {
    let mut depths: Vec<Vec<i64>> = Vec::default();
    depths.push(
        line.split_ascii_whitespace()
            .map(|num| num.parse::<i64>().expect("should be a number"))
            .collect::<Vec<i64>>(),
    );
    let mut row = 0;
    // build diff stack
    while !all_zero(&depths[row]) {
        let mut new_row: Vec<i64> = Vec::default();
        for i in 0..(depths[row].len() - 1) {
            new_row.push(depths[row][i + 1] - depths[row][i]);
        }
        depths.push(new_row);
        row += 1;
    }
    depths.reverse();
    depths
}

fn process_line_forwards(line: &str) -> i64 {
    let mut depths: Vec<Vec<i64>> = process_line(line);
    // append new values
    for i in 0..(depths.len()) {
        if i == 0 {
            depths[i].push(0);
        } else {
            let delta: i64 = depths[i - 1].last().expect("should not be empty").clone();
            let new_val: i64 = delta + (depths[i].last().expect("should not be empty").clone());
            depths[i].push(new_val);
        }
    }
    depths
        .last()
        .expect("should not be empty")
        .last()
        .expect("should not be empty")
        .clone()
}

fn process_line_backwards(line: &str) -> i64 {
    let mut depths: Vec<Vec<i64>> = process_line(line);
    for row in depths.iter_mut() {
        row.reverse();
    }
    // append new values
    for i in 0..(depths.len()) {
        if i == 0 {
            depths[i].push(0);
        } else {
            let delta: i64 = depths[i - 1].last().expect("should not be empty").clone();
            let new_val: i64 = (depths[i].last().expect("should not be empty").clone()) - delta;
            depths[i].push(new_val);
        }
    }
    depths
        .last()
        .expect("should not be empty")
        .last()
        .expect("should not be empty")
        .clone()
}

pub fn process_part1(input: &str) -> String {
    input
        .lines()
        .map(|line| process_line_forwards(line))
        .sum::<i64>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    input
        .lines()
        .map(|line| process_line_backwards(line))
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "114");
        assert_eq!(process_part2(file.as_str()), "2");
    }
}
