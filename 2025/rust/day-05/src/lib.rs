use rangemap::RangeInclusiveSet;

fn process_range(input: &str) -> (usize, usize) {
    let (left, right) = input.split_once("-").unwrap();
    (left.parse().unwrap(), right.parse().unwrap())
}

pub fn process_part1(input: &str) -> String {
    let mut coverage = RangeInclusiveSet::new();
    let mut range_mode = true;
    let mut fresh_count = 0;
    for line in input.lines() {
        if line.len() == 0 {
            range_mode = false;
            //for range in coverage.clone() {
            //    println!("{}-{}", range.start(), range.end());
            //}
            continue;
        }
        if range_mode {
            let (start, end) = process_range(line);
            coverage.insert(start..=end);
        } else {
            let id: usize = line.parse().unwrap();
            if coverage.contains(&id) {
                fresh_count += 1;
            } else {
                //println!("id: {id} spoiled!") // spoiled
            }
        }
    }
    fresh_count.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut coverage = RangeInclusiveSet::new();
    let mut fresh_count = 0;
    for line in input.lines() {
        if line.len() == 0 {
            //for range in coverage.clone() {
            //    println!("{}-{}", range.start(), range.end());
            //}
            break;
        }
        let (start, end) = process_range(line);
        coverage.insert(start..=end);
    }
    for range in coverage {
        fresh_count += range.end() - range.start() + 1;
    }
    fresh_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let file = include_str!("../test-input-1.txt");
        assert_eq!(process_part1(file), "3");
        assert_eq!(process_part2(file), "14");
    }
}
