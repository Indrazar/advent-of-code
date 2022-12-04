pub fn process_part1(input: &str) -> String {
    let result: usize = input
        .lines()
        .map(|line| {
            let line = line.split(',').collect::<Vec<&str>>();
            let left = line[0].split('-').collect::<Vec<&str>>();
            let right = line[1].split('-').collect::<Vec<&str>>();
            let left_start: usize = left[0].parse().unwrap();
            let left_end: usize = left[1].parse().unwrap();
            let right_start: usize = right[0].parse().unwrap();
            let right_end: usize = right[1].parse().unwrap();
            let left_size = left_end - left_start;
            let right_size = right_end - right_start;
            if left_size < right_size {
                if left_start >= right_start {
                    if left_end <= right_end { 1 } else { 0 }
                } else { 0 }
            } else {
                // right is bigger
                if right_start >= left_start {
                    if right_end <= left_end { 1 } else { 0 }
                } else { 0 }
            }
        })
        .sum::<usize>();
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let result: usize = input
        .lines()
        .map(|line| {
            let line = line.split(',').collect::<Vec<&str>>();
            let left = line[0].split('-').collect::<Vec<&str>>();
            let right = line[1].split('-').collect::<Vec<&str>>();
            let left_start: usize = left[0].parse().unwrap();
            let left_end: usize = left[1].parse().unwrap();
            let right_start: usize = right[0].parse().unwrap();
            let right_end: usize = right[1].parse().unwrap();
            if left_end < right_start || right_end < left_start { 0 } else { 1 }
        })
        .sum::<usize>();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_input() {
        let file = fs::read_to_string("./test-input-1.txt").unwrap();
        assert_eq!(process_part1(file.as_str()), "2");
        assert_eq!(process_part2(file.as_str()), "4");
    }
}
