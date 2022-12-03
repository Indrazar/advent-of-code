#![feature(iter_array_chunks)]

pub fn process_part1(input: &str) -> String {
    let result: usize = input
        .lines()
        .map(|line| {
            // we assume input is well-formed
            let (first, second) = line.split_at(line.chars().count() / 2);
            // we can do this because there is guaranteed to be exactly ONE collision
            let collision = first
                .chars()
                .find(|letter| second.contains(*letter))
                .unwrap();
            map_priority(collision)
        })
        .sum();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result: usize = input
        .lines()
        .array_chunks::<3>()
        .map(|[first, second, third]| {
            // we can do this because there is guaranteed to be exactly ONE collision
            let collision = first
                .chars()
                .find(|letter| second.contains(*letter) && third.contains(*letter))
                .unwrap();
            map_priority(collision)
        })
        .sum();
    result.to_string()
}

fn map_priority(input: char) -> usize {
    match input {
        'a'..='z' => input as usize - 'a' as usize + 1, // a = 1, b = 2, etc
        'A'..='Z' => input as usize - 'A' as usize + 27, // A = 27, B = 28, etc
        _ => todo!("invalid character found in map_priority"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "157");
        assert_eq!(process_part2(file.as_str()), "70");
    }
}
