use std::iter::zip;

pub fn process_part1(input: &str) -> String {
    let mut last_value: i32 = 0;
    let mut increases: i32 = 0;
    for (count, depth) in input.lines().enumerate() {
        let new_depth: i32 = depth.parse().expect("should be an i32");
        if count == 0 {
            last_value = new_depth;
        } else {
            if new_depth > last_value {
                increases += 1;
            }
            last_value = new_depth;
        }
    }
    increases.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut increases: i32 = 0;
    let depths: Vec<i32> = input
        .lines()
        .map(|l| l.parse::<i32>().expect("should be an i32"))
        .collect();
    let depths_iter1 = depths.windows(3);
    let mut depths_iter2 = depths.windows(3);
    depths_iter2.next();
    for (last, new) in zip(depths_iter1, depths_iter2) {
        if new.iter().sum::<i32>() > last.iter().sum::<i32>() {
            increases += 1
        }
    }

    increases.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "7");
        assert_eq!(process_part2(file), "5");
    }
}
